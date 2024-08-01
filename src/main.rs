use std::io::Error;
use std::io;
use std::io::Write;

use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
  #[arg(long)]
  gradient_length: usize,

  #[arg(long, value_parser, value_delimiter = ' ')]
  colors: Vec<String>,

  #[arg(long, default_value_t = false)]
  inline_colors: bool,
}

type Color<T> = (T, T, T);

fn main() -> Result<(), Error> {
  let args = Args::parse();

  assert!(
    args.gradient_length > args.colors.len(),
    "Gradient length must be greater than the color amount"
  );

  let hsl_colors: Vec<Color<f64>> = args.colors.iter()
    .map(|hex| hex_to_hsl(hex))
    .collect();

  let hsl_gradient = lerp_hsl(&hsl_colors, args.gradient_length);
  let hex_gradient: Vec<String> = hsl_gradient.iter()
    .map(|&(h, s, l)| hsl_to_hex(h, s, l))
    .collect();

  for hex_color in hex_gradient {
    print_hex_color(&hex_color, args.inline_colors);
  }

  Ok(())
}

fn print_hex_color(hex: &str, inline: bool) {
  let (r, g, b) = hex_to_rgb(&hex);
  let colored_text = hex.truecolor(r, g, b);

  if inline {
    print!("{} ", colored_text);
    io::stdout().flush().unwrap();
    return;
  }

  let colored_rgb_text = format!("({}, {}, {})", r, g, b).truecolor(r, g, b);
  let colored_background = " ".repeat(5).on_truecolor(r, g, b);

  println!("{} {} {}", colored_text, colored_background, colored_rgb_text);
}

fn hex_to_rgb(hex: &str) -> Color<u8> {
  let hex = hex.trim_matches('#');
  let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
  let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
  let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

  (r, g, b)
}

fn hex_to_hsl(hex: &str) -> Color<f64> {
  let (r, g, b) = hex_to_rgb(hex);
  let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);

  let max = r.max(g).max(b);
  let min = r.min(g).min(b);
  let range = max - min;

  let l = (max + min) / 2.0;

  let (h, s) = match range {
    0.0 => (0.0, 0.0),
    _ => {
      let s = match l > 0.5 {
        true => range / (2.0 - max - min),
        false => range / (max + min),
      };

      let h = match max {
        max if max == r => (g - b) / range + if g < b { 6.0 } else { 0.0 },
        max if max == g => (b - r) / range + 2.0,
        _ => (r - g) / range + 4.0,
      } / 6.0;
  
      (h, s)
    },
  };

  (h * 360.0, s * 100.0, l * 100.0)
}

fn hsl_to_hex(h: f64, s: f64, l: f64) -> String {
  let h = h / 360.0;
  let s = s / 100.0;
  let l = l / 100.0;

  let (r, g, b) = match s {
    s if s == 0.0 => (l, l, l),
    _ => {
      let q = if l < 0.5 {
        l * (1.0 + s)
      } else {
        l + s - l * s
      };

      let p = 2.0 * l - q;

      (hue_to_rgb(p, q, h + 1.0 / 3.0), hue_to_rgb(p, q, h), hue_to_rgb(p, q, h - 1.0 / 3.0))
    }
  };

  format!("#{:02X}{:02X}{:02X}", (r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
  let t = match t {
    t if t < 0.0 => t + 1.0,
    t if t > 1.0 => t - 1.0,
    _ => t,
  };

  match t {
    t if t < 1.0 / 6.0 => p + (q - p) * 6.0 * t,
    t if t < 1.0 / 2.0 => q,
    t if t < 2.0 / 3.0 => p + (q - p) * (2.0 / 3.0 - t) * 6.0,
    _ => p,
  }
}

fn lerp_hsl(colors: &[Color<f64>], n: usize) -> Vec<Color<f64>> {
  let mut result = Vec::with_capacity(n);
  let segments = colors.len() - 1;
  let colors_per_segment = (n - 1) / segments;

  for i in 0..segments {
    let (h1, s1, l1) = colors[i];
    let (h2, s2, l2) = colors[i + 1];

    for j in 0..colors_per_segment {
      let t = j as f64 / colors_per_segment as f64;
      let h = h1 + t * (h2 - h1);
      let s = s1 + t * (s2 - s1);
      let l = l1 + t * (l2 - l1);
      result.push((h, s, l));
    }
  }

  result.push(*colors.last().unwrap());

  result
}
