use std::collections::VecDeque;

use rship_sdk::{EmitterArgs, EmitterProxy, InstanceProxy, TargetArgs, TargetProxy};

use crate::{hardware::Momentary, util::IntoResult};

#[derive(Clone)]
pub struct CueInfo {
    pub number: String,
    pub name: String,
    pub sequence_name: String,
}

pub fn parse_cue_info(cue_slug: String) -> anyhow::Result<CueInfo, anyhow::Error> {
    let mut segments = cue_slug
        .split(" ")
        .into_iter()
        .filter(|x| x.len() > 0)
        .collect::<VecDeque<_>>();

    let seq_name = segments.pop_front().into_result()?;

    let cue_number = *segments.front().into_result()?;

    let cue_name = segments.into_iter().collect::<Vec<_>>().join(" ");

    Ok(CueInfo {
        number: cue_number.into(),
        name: cue_name,
        sequence_name: seq_name.into(),
    })
}

pub struct Cue {
    // pub proxy: TargetProxy,
    // pub go_plus: EmitterProxy<Momentary>,
    // pub go_minus: EmitterProxy<Momentary>,
    // pub goto: EmitterProxy<Momentary>,
    // pub jump_forward: EmitterProxy<Momentary>,
    // pub jump_backward: EmitterProxy<Momentary>,
}

impl Cue {
    pub async fn new(
        sequence_id: i32,
        cue_info: &CueInfo,
        // parent: &TargetProxy,
        // instance: &InstanceProxy,
    ) -> Self {
        let short_id = format!("{} {}", sequence_id, cue_info.number.clone());

        println!("Creating cue: {}", short_id);

        // let mut cue_proxy = instance
        //     .add_target(TargetArgs {
        //         category: "Cue".into(),
        //         name: format!(
        //             "[{}] - {}",
        //             cue_info.number,
        //             cue_info.name.replace(&cue_info.number, "").trim(),
        //         ),
        //         parent_targets: Some(vec![parent.clone()]),
        //         short_id,
        //     })
        //     .await;

        // let go_plus = cue_proxy
        //     .add_emitter::<Momentary>(EmitterArgs::new(
        //         "Go+".to_string(),
        //         format!("Go+ {}", cue_info.number.clone()),
        //     ))
        //     .await;

        // let go_minus = cue_proxy
        //     .add_emitter::<Momentary>(EmitterArgs::new(
        //         "Go-".to_string(),
        //         format!("Go- {}", cue_info.number.clone()),
        //     ))
        //     .await;

        // let goto = cue_proxy
        //     .add_emitter::<Momentary>(EmitterArgs::new(
        //         "Goto".to_string(),
        //         format!("Goto {}", cue_info.number.clone()),
        //     ))
        //     .await;

        // let jump_forward = cue_proxy
        //     .add_emitter::<Momentary>(EmitterArgs::new(
        //         "JumpForward".to_string(),
        //         format!("JumpForward {}", cue_info.number.clone()),
        //     ))
        //     .await;

        // let jump_backward = cue_proxy
        //     .add_emitter::<Momentary>(EmitterArgs::new(
        //         "JumpBackward".to_string(),
        //         format!("JumpBackward {}", cue_info.number.clone()),
        //     ))
        //     .await;

        Self {
            // proxy: cue_proxy,
            // go_plus,
            // go_minus,
            // goto,
            // jump_forward,
            // jump_backward,
        }
    }
}
