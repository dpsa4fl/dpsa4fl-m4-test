

use dpsa4fl::{controller::*, core::{CommonState_Parametrization, Locations}, client::{api__new_client_state, api__submit, RoundSettings}};
use fixed_macro::fixed;
use fixed::types::I1F31;
use url::Url;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Submitting gradient with 3 elements.");
    run_aggregation(3, fixed!(0.0625: I1F31)).await?;
    println!("\n");

    println!("Submitting gradient with 60000 elements.");
    run_aggregation(60000, fixed!(0.0: I1F31)).await?;

    Ok(())
}

struct PrintShortVec<'a, A>(&'a Vec<A>);

impl<'a, A : std::fmt::Debug + Clone> std::fmt::Display for PrintShortVec<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub_size = core::cmp::min(self.0.len(), 15); // don't print the whole vector if it is too long
        write!(f, "{:?}", (self.0)[0..sub_size].to_vec())
    }
}


// run the full aggregation pipeline with gradient vectors
// of length `gradient_len`, filled with elements `value`.
async fn run_aggregation(gradient_len: usize, value: I1F31) -> Result<()> {

    // Create controller and prepare aggregation.
    println!("Creating controller");
    let location = Locations {
            internal_leader: Url::parse("http://aggregator1:9991")?,
            internal_helper: Url::parse("http://aggregator2:9992")?,
            external_leader_tasks: Url::parse("http://127.0.0.1:9981")?,
            external_helper_tasks: Url::parse("http://127.0.0.1:9982")?,
            external_leader_main: Url::parse("http://127.0.0.1:9991")?,
            external_helper_main: Url::parse("http://127.0.0.1:9992")?,
    };

    let p = CommonState_Parametrization {
        location,
        gradient_len,
        noise_parameter: (10000,1),
    };
    let istate = api__new_controller_state(p.clone());
    let mut mstate = ControllerState_Mut {
        round: ControllerState_Round { task_id: None, training_session_id: None }
    };
    api__create_session(&istate, &mut mstate).await?;
    let task_id = api__start_round(&istate,&mut mstate).await?;
    println!("started round with task id {task_id}");

    // Submitting a gradient, has to be done by each client
    async fn submit_gradient(task_id: String, p: CommonState_Parametrization, gradient_len: usize, value: I1F31) -> Result<()> {
        let round_settings : RoundSettings = RoundSettings::new(task_id)?;
        let data = vec![value; gradient_len];
        println!("submitting vector: {}", PrintShortVec(&data));

        let mut state = api__new_client_state(p.clone());
        api__submit(&mut state, round_settings, &data).await?;

        Ok(())
    }

    // Call the submission function
    println!("submitting gradient 1");
    submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;
    println!("submitting gradient 2");
    submit_gradient(task_id.clone(), p.clone(), gradient_len, value).await?;

    // Wait for janus to aggregate, and get result
    println!("collecting");
    let res = api__collect(&istate,&mut mstate).await?;
    let val = res.aggregate_result();
    let sub_size = core::cmp::min(val.len(), 15); // don't print the whole vector if it is too long
    println!("got result, it is:\n{}", PrintShortVec(val));

    // Done
    Ok(())
}



