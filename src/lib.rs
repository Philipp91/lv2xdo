use std::time::{Duration, SystemTime};

use lv2::prelude::*;
use wmidi::{MidiMessage, U14};

use xdo::XDo;

mod xdo;

const U14_MIDDLE: u16 = 1 << 13;
const U14_MAX: u16 = (1 << 14) - 1;

#[derive(PortCollection)]
struct Ports {
    #[allow(dead_code)]
    midi_in: InputPort<AtomPort>,
    control_play: InputPort<Control>,
    control_pause: InputPort<Control>,
    control_stop: InputPort<Control>,
    control_record: InputPort<Control>,
    control_prev: InputPort<Control>,
    control_next: InputPort<Control>,
    control_rewind: InputPort<Control>,
    control_forward: InputPort<Control>,
    control_repeat: InputPort<Control>,
    control_lower_volume: InputPort<Control>,
    control_raise_volume: InputPort<Control>,
    control_mute: InputPort<Control>,
    control_media: InputPort<Control>,
    enable_pitch_to_seek: InputPort<Control>,
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
        make_key_mapping(|ports: &Ports| &ports.control_pause, "XF86AudioPause"),
        make_key_mapping(|ports: &Ports| &ports.control_stop, "XF86AudioStop"),
        make_key_mapping(|ports: &Ports| &ports.control_record, "XF86AudioRecord"),
        make_key_mapping(|ports: &Ports| &ports.control_prev, "XF86AudioPrev"),
        make_key_mapping(|ports: &Ports| &ports.control_next, "XF86AudioNext"),
        make_key_mapping(|ports: &Ports| &ports.control_rewind, "XF86AudioRewind"),
        make_key_mapping(|ports: &Ports| &ports.control_forward, "XF86AudioForward"),
        make_key_mapping(|ports: &Ports| &ports.control_repeat, "XF86AudioRepeat"),
        make_key_mapping(|ports: &Ports| &ports.control_lower_volume, "XF86AudioLowerVolume"),
        make_key_mapping(|ports: &Ports| &ports.control_raise_volume, "XF86AudioRaiseVolume"),
        make_key_mapping(|ports: &Ports| &ports.control_mute, "XF86AudioMute"),
        make_key_mapping(|ports: &Ports| &ports.control_media, "XF86AudioMedia"),
    ]
}

#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: LV2Map<'a>,
}

#[derive(URIDCollection)]
pub struct URIDs {
    atom: AtomURIDCollection,
    midi: MidiURIDCollection,
    unit: UnitURIDCollection,
}

#[uri("https://philippkeck.de/lv2/lv2xdo")]
struct LV2XDO {
    urids: URIDs,
    xdo: XDo,
    key_mappings: Vec<KeyMapping>,
    pitch_value: u16,
    last_seek_time: SystemTime,
}

impl Plugin for LV2XDO {
    type Ports = Ports;
    type InitFeatures = Features<'static>;
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, features: &mut Features<'static>) -> Option<Self> {
        Some(Self {
            urids: features.map.populate_collection()?,
            xdo: XDo::new(None).unwrap(),
            key_mappings: default_key_mappings(),
            pitch_value: U14_MIDDLE,
            last_seek_time: SystemTime::now(),
        })
    }

    fn run(&mut self, ports: &mut Ports, _: &mut (), _: u32) {
        self.run_key_mappings(ports);

        if *ports.enable_pitch_to_seek > 0.5 {
            self.run_pitch_bend_to_seek(ports);
        }
    }
}

impl LV2XDO {
    // Map boolean control inputs to simple keypresses when they flip from 0 to 1.
    fn run_key_mappings(&mut self, ports: &Ports) {
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

    // Given a `seek_delay` between 0 (fastest seeking) and U14_MIDDLE (slowest seeking), this
    // functions returns true if the seek should happen right now based on when it last happened.
    fn should_seek_now(&mut self, seek_delay: u16) -> bool {
        let now = SystemTime::now();
        let delay = Duration::from_micros(500000u64 * (seek_delay as u64) / (U14_MAX as u64));
        if self.last_seek_time < now - delay {
            self.last_seek_time = now;
            return true;
        }
        return false;
    }

    fn run_pitch_bend_to_seek(&mut self, ports: &Ports) {
        let input_sequence = ports.midi_in.read(self.urids.atom.sequence, self.urids.unit.beat).unwrap();
        for (_, atom) in input_sequence {
            if let Some(MidiMessage::PitchBendChange(_, pitch_bend)) = atom.read(self.urids.midi.wmidi, ()) {
                self.pitch_value = U14::data_to_slice(&[pitch_bend])[0];
                break;
            }
        }

        if self.pitch_value > U14_MIDDLE && self.should_seek_now(U14_MAX - self.pitch_value) {
            self.xdo.send_keysequence("Right", 0).ok();
        } else if self.pitch_value < U14_MIDDLE && self.should_seek_now(self.pitch_value) {
            self.xdo.send_keysequence("Left", 0).ok();
        }
    }
}

lv2_descriptors!(LV2XDO);
