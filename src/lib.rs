mod xdo;

use lv2::prelude::*;
use xdo::XDo;

#[derive(PortCollection)]
struct Ports {
    #[allow(dead_code)]
    midi_in: InputPort<AtomPort>,
    control_play: InputPort<Control>,
}

struct KeyMapping {
    input_port: fn(&Ports) -> &InputPort<Control>,
    last_state: bool,
    keysequence: &'static str,
}

fn make_key_mapping(input_port: fn(&Ports) -> &InputPort<Control>,
                    keysequence: &'static str) -> KeyMapping {
    return KeyMapping { input_port, last_state: false, keysequence };
}

fn default_key_mappings() -> Vec<KeyMapping> {
    vec![
        make_key_mapping(|ports: &Ports| &ports.control_play, "XF86AudioPlay"),
    ]
}

#[uri("https://philippkeck.de/lv2/lv2xdo")]
struct LV2XDO {
    xdo: XDo,
    key_mappings: Vec<KeyMapping>,
}

impl Plugin for LV2XDO {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            xdo: XDo::new(None).unwrap(),
            key_mappings: default_key_mappings(),
        })
    }

    fn run(&mut self, ports: &mut Ports, _: &mut (), _: u32) {
        for key_mapping in &mut self.key_mappings {
            if **((key_mapping.input_port)(ports)) > 0.5 {
                if !key_mapping.last_state {
                    key_mapping.last_state = true;
                    self.xdo.send_keysequence(key_mapping.keysequence, 0).ok();
                }
            } else {
                if key_mapping.last_state {
                    key_mapping.last_state = false;
                }
            }
        }
    }
}

lv2_descriptors!(LV2XDO);
