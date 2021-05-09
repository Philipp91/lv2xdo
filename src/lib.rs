use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
    midi_in: InputPort<AtomPort>,
    control_play: InputPort<Control>,
}

#[uri("https://philippkeck.de/lv2/lv2xdo")]
struct LV2XDO;

impl Plugin for LV2XDO {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    fn run(&mut self, ports: &mut Ports, _: &mut (), _: u32) {
    }
}

lv2_descriptors!(LV2XDO);
