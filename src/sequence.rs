use std::collections::HashMap;

use rosc::OscType;
use rship_sdk::{EmitterArgs, EmitterProxy, InstanceProxy, TargetArgs, TargetProxy};

use crate::{
    cue::{Cue, CueInfo},
    hardware::{Fader, Momentary},
    util::IntoResult,
};
pub struct Sequence {
    id: i32,
    cues: HashMap<String, Cue>,
    pub proxy: TargetProxy,
    pub pause: EmitterProxy<Momentary>,
    faders: HashMap<String, EmitterProxy<Fader>>,
}

use anyhow::{Error, Ok, Result};

impl Sequence {
    pub async fn new(seq_number: i32, instance: &InstanceProxy) -> Self {
        let mut seq_proxy = instance
            .add_target(TargetArgs {
                category: "Sequence".into(),
                name: format!("Sequence {}", seq_number),
                parent_targets: None,
                short_id: seq_number.to_string(),
            })
            .await;

        let pause_proxy = seq_proxy
            .add_emitter::<Momentary>(EmitterArgs::new(
                "Pause".to_string(),
                format!("Pause {}", seq_number),
            ))
            .await;

        Self {
            id: seq_number,
            cues: HashMap::new(),
            proxy: seq_proxy,
            pause: pause_proxy,
            faders: HashMap::new(),
        }
    }

    pub async fn rename(&mut self, name: String) {
        self.proxy.rename(name).await;
    }

    pub async fn register_cue(
        &mut self,
        cue_info: &CueInfo,
        instance: &InstanceProxy,
    ) -> anyhow::Result<(), anyhow::Error> {
        if self.cues.contains_key(cue_info.number.as_str()) {
            return Ok(());
        }

        let cue = Cue::new(self.id, cue_info, &self.proxy, instance).await;

        self.cues.insert(cue_info.number.clone().into(), cue);

        self.rename(cue_info.sequence_name.clone().into()).await;

        Ok(())
    }

    pub async fn go_plus(&mut self, cue_info: CueInfo) -> anyhow::Result<(), anyhow::Error> {
        let cue = self.cues.get_mut(&cue_info.number).into_result()?;

        cue.proxy.rename(cue_info.name.clone()).await;

        let _ = cue
            .go_plus
            .pulse(Momentary {
                id: uuid::Uuid::new_v4().to_string(),
                tag: None,
            })
            .await;

        Ok(())
    }

    pub async fn go_minus(&mut self, cue_info: CueInfo) -> anyhow::Result<(), anyhow::Error> {
        let cue = self.cues.get_mut(&cue_info.number).into_result()?;

        cue.proxy.rename(cue_info.name.clone()).await;

        let _ = cue
            .go_minus
            .pulse(Momentary {
                id: uuid::Uuid::new_v4().to_string(),
                tag: None,
            })
            .await;

        Ok(())
    }

    pub async fn goto(&mut self, cue_info: CueInfo) -> anyhow::Result<(), anyhow::Error> {
        let cue = self.cues.get_mut(&cue_info.number).into_result()?;

        cue.proxy.rename(cue_info.name.clone()).await;

        let _ = cue
            .goto
            .pulse(Momentary {
                id: uuid::Uuid::new_v4().to_string(),
                tag: None,
            })
            .await;

        Ok(())
    }
    pub async fn jump_forward(&mut self, cue_info: CueInfo) -> anyhow::Result<(), anyhow::Error> {
        let cue = self.cues.get_mut(&cue_info.number).into_result()?;

        cue.proxy.rename(cue_info.name.clone()).await;

        let _ = cue
            .jump_forward
            .pulse(Momentary {
                id: uuid::Uuid::new_v4().to_string(),
                tag: None,
            })
            .await;

        Ok(())
    }
    pub async fn jump_backward(&mut self, cue_info: CueInfo) -> anyhow::Result<(), anyhow::Error> {
        let cue = self.cues.get_mut(&cue_info.number).into_result()?;

        cue.proxy.rename(cue_info.name.clone()).await;

        let _ = cue
            .jump_backward
            .pulse(Momentary {
                id: uuid::Uuid::new_v4().to_string(),
                tag: None,
            })
            .await;

        Ok(())
    }

    pub async fn process_fader(
        &mut self,
        fader_functon: &String,
        args: &Vec<OscType>,
    ) -> Result<(), Error> {
        if !fader_functon.starts_with("Fader") {
            return Err(Error::msg("Not a fader"));
        }

        if args.len() < 2 {
            return Err(Error::msg("Not enough args"));
        }

        let level = args[2].clone().float().into_result()?;

        let short_id = format!("{}{}", self.id, fader_functon);
        if !self.faders.contains_key(&short_id) {
            println!("Creating Fader Proxy {}", short_id);

            let proxy = self
                .proxy
                .add_emitter(EmitterArgs::<Fader>::new(
                    fader_functon.clone(),
                    short_id.clone(),
                ))
                .await;

            self.faders.insert(short_id.clone(), proxy);
        }

        let fader = self.faders.get(&short_id).into_result()?;

        let _ = fader
            .pulse(Fader {
                level: level,
                tag: None,
            })
            .await;

        Ok(())
    }
}
