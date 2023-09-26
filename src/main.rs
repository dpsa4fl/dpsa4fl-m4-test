

use dpsa4fl::{core::{types::{Locations, MainLocations, ManagerLocations, CommonStateParametrization, VdafParameter}, fixed::{VecFixedAny, FixedTypeTag}}, controller::interface::{embedded::{api_new_controller_state, api_create_session, api_start_round, api_collect}, types::{ControllerStateMut, ControllerStateRound}}, client::interface::{types::RoundSettings, embedded::{api_new_client_state, api_submit_with}}};
// use dpsa4fl::{controller::*, core::{CommonState_Parametrization, Locations}, client::{api__new_client_state, api__submit, RoundSettings}};
use fixed_macro::fixed;
use fixed::{types::{I1F31, I1F15, I1F63}, traits::Fixed};
use url::Url;
use anyhow::Result;
use prio::{dp::Rational, vdaf::{self, prio3::{self, Prio3}}, flp::{types::fixedpoint_l2::FixedPointBoundedL2VecSum, Type, gadgets::{ParallelSum, PolyEval, Mul}}, field::Field128};
use prio::dp::ZCdpBudget;

type Fx = I1F31;

#[tokio::main]
async fn main() -> Result<()> {

    ////////////////////////////////////////////////////////
    // testing small vectors
    // let n = 1;
    // println!("Submitting gradient with {n} elements.");
    // run_aggregation(n, fixed!(0.9: I1F15)).await?;

    ////////////////////////////////////////////////////////
    // testing large vectors
    let n = 1<<18;
    println!("Submitting gradient with {n} elements.");
    run_aggregation(n, fixed!(0.0: I1F31)).await?;


    ////////////////////////////////////////////////////////
    // for i in 10..18 {
    //     let size = get_size(1 << i);
    //     let encoded_bytes = size*(128/8);
    //     let kib = encoded_bytes / 1024;
    //     println!("2^{i} => {size} entries => {encoded_bytes} bytes => {kib} kib");
    // }

    Ok(())
}

fn get_size(length: usize) -> usize
{
    let typ :
    FixedPointBoundedL2VecSum<
            Fx,
        ParallelSum<Field128, PolyEval<Field128>>,
        ParallelSum<Field128, Mul<Field128>>,
        >
    = FixedPointBoundedL2VecSum::new(length).unwrap();

    typ.input_len() + typ.proof_len()
}

// Struct wrapper for convenient debug printing.
struct PrintShortVec<'a, A>(&'a Vec<A>);

impl<'a, A : std::fmt::Debug + Clone> std::fmt::Display for PrintShortVec<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_size = core::cmp::min(self.0.len(), 15); // don't print the whole vector if it is too long
        write!(f, "{:?}", (self.0)[0..sub_size].to_vec())
    }
}


// run the full aggregation pipeline with gradient vectors
// of length `gradient_len`, filled with elements `value`.
async fn run_aggregation(gradient_len: usize, value: Fx) -> Result<()> {

    // Create controller and prepare aggregation.
    println!("Creating controller");
    let location = Locations {
            // internal_leader: Url::parse("http://aggregator1:9991")?,
            // internal_helper: Url::parse("http://aggregator2:9992")?,
            // external_leader_tasks: Url::parse("http://127.0.0.1:9981")?,
            // external_helper_tasks: Url::parse("http://127.0.0.1:9982")?,
            // external_leader_main: Url::parse("http://127.0.0.1:9991")?,
            // external_helper_main: Url::parse("http://127.0.0.1:9992")?,
        main: MainLocations
        {
            external_leader: Url::parse("http://127.0.0.1:9991")?,
            external_helper: Url::parse("http://127.0.0.1:9992")?
        },
        manager: ManagerLocations
        {
            external_leader: Url::parse("http://127.0.0.1:9981")?,
            external_helper: Url::parse("http://127.0.0.1:9982")?,
        },
    };

    let p = CommonStateParametrization {
        location,
        // gradient_len,
        // noise_parameter: (10000,1),
        vdaf_parameter: VdafParameter {
            gradient_len,
            privacy_parameter: ZCdpBudget::new(Rational::try_from(100.0f32)?),
            submission_type: FixedTypeTag::FixedType32Bit,
        },
    };
    let istate = api_new_controller_state(p.clone());
    let mut mstate = ControllerStateMut {
        round: ControllerStateRound { task_id: None, training_session_id: None }
    };
    api_create_session(&istate, &mut mstate).await?;
    let task_id = api_start_round(&istate,&mut mstate).await?;
    println!("started round with task id {task_id}");

    // Submitting a gradient, has to be done by each client
    async fn submit_gradient(task_id: String, p: CommonStateParametrization, gradient_len: usize, value: Fx) -> Result<()>
        where Fx : Clone + std::fmt::Debug
    {
        let round_settings : RoundSettings = RoundSettings::new(task_id)?;
        let data = vec![value; gradient_len];
        println!("submitting vector: {}", PrintShortVec(&data));

        let mut state = api_new_client_state(p.location.manager);
        api_submit_with(&mut state, round_settings, |x| VecFixedAny::VecFixed32(data)).await?;
        Ok(())
    }

    println!("press enter to send.");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    // Call the submission function
    println!("submitting gradient 1");
    submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;
    println!("submission finished");

    println!("press enter to continue.");

    std::io::stdin().read_line(&mut line).unwrap();

    println!("submitting gradient 2");
    submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;

    // println!("press enter to continue.");

    //           std::io::stdin().read_line(&mut line).unwrap();

    // println!("submitting gradient 3");
    // submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;

    // println!("press enter to continue.");

    //           std::io::stdin().read_line(&mut line).unwrap();

    // println!("submitting gradient 4");
    // submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;

    // Wait for janus to aggregate, and get result
    println!("collecting");
    let res = api_collect(&istate,&mut mstate).await?;
    let val = res.aggregate_result();
    // let sub_size = core::cmp::min(val.len(), 15); // don't print the whole vector if it is too long
    println!("got result, it is:\n{}", PrintShortVec(val));
    // println!("got result, it is:\n{:?}", val);

    // Done
    Ok(())
}



