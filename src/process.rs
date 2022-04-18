use colors_transform::Rgb;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Process {
    pub mem_size: i32,
    pub mem_position: i32,
    pub pid: i32,
    pub runtime: i32,
    pub block_color: Rgb, //(f32, f32, f32),
}

pub fn generate_process_list() -> Vec<Process> {
    let colors = vec![
        Rgb::from(163.20001, 244.79999, 195.80103),
        Rgb::from(244.79999, 163.20001, 190.19577),
        Rgb::from(163.20001, 244.79999, 243.70969),
        Rgb::from(163.20001, 209.6547, 244.79999),
        Rgb::from(163.20001, 244.79999, 166.1844),
        Rgb::from(244.79999, 163.20001, 198.9105),
        Rgb::from(163.20001, 243.03578, 244.79999),
        Rgb::from(244.79999, 210.17773, 163.20001),
        Rgb::from(244.79999, 163.20001, 209.64311),
        Rgb::from(244.79999, 168.32787, 163.20001),
        Rgb::from(163.20001, 244.79999, 204.3005),
        Rgb::from(244.79999, 163.20001, 169.22879),
        Rgb::from(244.79999, 163.20001, 232.11832),
        Rgb::from(163.20001, 204.22829, 244.79999),
        Rgb::from(244.79999, 163.20001, 189.29393),
        Rgb::from(244.79999, 210.84409, 163.20001),
        Rgb::from(206.58409, 244.79999, 163.20001),
        Rgb::from(244.79999, 223.19739, 163.20001),
        Rgb::from(183.4302, 163.20001, 244.79999),
        Rgb::from(244.79999, 214.50961, 163.20001),
        Rgb::from(163.20001, 244.79999, 212.55818),
        Rgb::from(244.79999, 204.30655, 163.20001),
        Rgb::from(244.79999, 163.20001, 176.73187),
        Rgb::from(244.79999, 233.97554, 163.20001),
        Rgb::from(244.79999, 192.72272, 163.20001),
        Rgb::from(163.20001, 244.79999, 172.5628),
        Rgb::from(163.20001, 244.79999, 237.06625),
        Rgb::from(244.79999, 163.20001, 212.89835),
        Rgb::from(207.6975, 163.20001, 244.79999),
        Rgb::from(163.20001, 181.06786, 244.79999),
        Rgb::from(244.79999, 180.55707, 163.20001),
        Rgb::from(244.79999, 163.20001, 206.59494),
        Rgb::from(163.20001, 173.69963, 244.79999),
    ];

    vec![
        Process {
            pid: 30,
            mem_size: 3,
            mem_position: -1,
            runtime: 5,
            block_color: *colors.get(30).unwrap(),
        },
        Process {
            pid: 29,
            mem_size: 3,
            mem_position: -1,
            runtime: 5,
            block_color: *colors.get(29).unwrap(),
        },
        Process {
            pid: 28,
            mem_size: 9,
            mem_position: -1,
            runtime: 5,
            block_color: *colors.get(28).unwrap(),
        },
        Process {
            pid: 27,
            mem_size: 2,
            mem_position: -1,
            runtime: 5,
            block_color: *colors.get(27).unwrap(),
        },
        Process {
            pid: 26,
            mem_size: 3,
            mem_position: -1,
            runtime: 4,
            block_color: *colors.get(26).unwrap(),
        },
        Process {
            pid: 25,
            mem_size: 4,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(25).unwrap(),
        },
        Process {
            pid: 24,
            mem_size: 5,
            mem_position: -1,
            runtime: 2,
            block_color: *colors.get(24).unwrap(),
        },
        Process {
            pid: 23,
            mem_size: 1,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(23).unwrap(),
        },
        Process {
            pid: 22,
            mem_size: 1,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(22).unwrap(),
        },
        Process {
            pid: 21,
            mem_size: 4,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(21).unwrap(),
        },
        Process {
            pid: 20,
            mem_size: 3,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(20).unwrap(),
        },
        Process {
            pid: 19,
            mem_size: 2,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(19).unwrap(),
        },
        Process {
            pid: 18,
            mem_size: 1,
            mem_position: -1,
            runtime: 7,
            block_color: *colors.get(18).unwrap(),
        },
        Process {
            pid: 17,
            mem_size: 2,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(17).unwrap(),
        },
        Process {
            pid: 16,
            mem_size: 20,
            mem_position: -1,
            runtime: 8,
            block_color: *colors.get(16).unwrap(),
        },
        Process {
            pid: 15,
            mem_size: 8,
            mem_position: -1,
            runtime: 6,
            block_color: *colors.get(15).unwrap(),
        },
        Process {
            pid: 14,
            mem_size: 1,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(14).unwrap(),
        },
        Process {
            pid: 13,
            mem_size: 1,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(13).unwrap(),
        },
        Process {
            pid: 12,
            mem_size: 1,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(12).unwrap(),
        },
        Process {
            pid: 11,
            mem_size: 1,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(11).unwrap(),
        },
        Process {
            pid: 10,
            mem_size: 1,
            mem_position: -1,
            runtime: 3,
            block_color: *colors.get(10).unwrap(),
        },
        Process {
            pid: 9,
            mem_size: 1,
            mem_position: -1,
            runtime: 8,
            block_color: *colors.get(9).unwrap(),
        },
        Process {
            pid: 8,
            mem_size: 2,
            mem_position: -1,
            runtime: 34,
            block_color: *colors.get(8).unwrap(),
        },
        Process {
            pid: 7,
            mem_size: 3,
            mem_position: -1,
            runtime: 4,
            block_color: *colors.get(7).unwrap(),
        },
        Process {
            pid: 6,
            mem_size: 2,
            mem_position: -1,
            runtime: 36,
            block_color: *colors.get(6).unwrap(),
        },
        Process {
            pid: 5,
            mem_size: 1,
            mem_position: -1,
            runtime: 6,
            block_color: *colors.get(5).unwrap(),
        },
        Process {
            pid: 4,
            mem_size: 8,
            mem_position: -1,
            runtime: 38,
            block_color: *colors.get(4).unwrap(),
        },
        Process {
            pid: 3,
            mem_size: 2,
            mem_position: -1,
            runtime: 8,
            block_color: *colors.get(3).unwrap(),
        },
        Process {
            pid: 2,
            mem_size: 2,
            mem_position: -1,
            runtime: 40,
            block_color: *colors.get(2).unwrap(),
        },
        Process {
            pid: 1,
            mem_size: 1,
            mem_position: -1,
            runtime: 10,
            block_color: *colors.get(1).unwrap(),
        },
    ]
}