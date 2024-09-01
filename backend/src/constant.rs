pub const X: [f64; 90] = {
    let mut x: [f64; 90] = [0.0; 90];

    let mut i = 0;
    while i < 73 {
        x[i] = 0.006;
        i += 1;
    }

    while i < 89 {
        x[i] = 0.006 + ((i - 73) as f64) * 0.06;
        i += 1;
    }

    x[89] = 1.0;

    x
};

pub const Y: [f64; 77] = {
    let mut y: [f64; 77] = [0.0; 77];

    let mut i = 0;
    while i < 62 {
        y[i] = 0.007;
        i += 1;
    }

    while i < 76 {
        y[i] = 0.007 + ((i - 62 + 1) as f64) * 0.07;
        i += 1;
    }

    y[76] = 1.0;

    y
};
