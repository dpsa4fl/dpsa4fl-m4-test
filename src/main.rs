

use dpsa4fl::{controller::*, core::{CommonState_Parametrization, Locations}, client::{api__new_client_state, api__submit, RoundSettings}};
use fixed_macro::fixed;
use url::Url;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
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
        gradient_len: 3,
        noise_parameter: (10000,1),
    };
    let istate = api__new_controller_state(p.clone());
    let mut mstate = ControllerState_Mut {
        round: ControllerState_Round { task_id: None, training_session_id: None }
    };
    api__create_session(&istate, &mut mstate).await?;
    let task_id = api__start_round(&istate,&mut mstate).await?;
    println!("started round with task id {task_id}");

    async fn submit_gradient(task_id: String, p: CommonState_Parametrization) -> Result<()> {
        let round_settings : RoundSettings = RoundSettings::new(task_id)?;
        let f = fixed!(0.0625: I1F31);
        let data = vec![f, f, f];
        println!("submitting vector: {data:?}");

        let mut state = api__new_client_state(p.clone());
        api__submit(&mut state, round_settings, &data).await?;

        Ok(())
    }

    println!("submitting gradient 1");
    submit_gradient(task_id.clone(), p.clone()).await?;
    println!("submitting gradient 2");
    submit_gradient(task_id.clone(), p.clone()).await?;


    // wait for result
    println!("collecting");
    let res = api__collect(&istate,&mut mstate).await?;
    let val = res.aggregate_result();
    println!("got result, it is:\n{val:?}");

    Ok(())
}



