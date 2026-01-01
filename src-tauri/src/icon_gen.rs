use tauri::image::Image;

const DIGITS: [[[u8; 3]; 5]; 10] = [
    [
        // 0
        [1, 1, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 0, 1],
        [1, 1, 1],
    ],
    [
        // 1
        [0, 1, 0],
        [1, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [1, 1, 1],
    ],
    [
        // 2
        [1, 1, 1],
        [0, 0, 1],
        [1, 1, 1],
        [1, 0, 0],
        [1, 1, 1],
    ],
    [
        // 3
        [1, 1, 1],
        [0, 0, 1],
        [1, 1, 1],
        [0, 0, 1],
        [1, 1, 1],
    ],
    [
        // 4
        [1, 0, 1],
        [1, 0, 1],
        [1, 1, 1],
        [0, 0, 1],
        [0, 0, 1],
    ],
    [
        // 5
        [1, 1, 1],
        [1, 0, 0],
        [1, 1, 1],
        [0, 0, 1],
        [1, 1, 1],
    ],
    [
        // 6
        [1, 1, 1],
        [1, 0, 0],
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1],
    ],
    [
        // 7
        [1, 1, 1],
        [0, 0, 1],
        [0, 0, 1],
        [0, 1, 0],
        [0, 1, 0],
    ],
    [
        // 8
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1],
    ],
    [
        // 9
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1],
        [0, 0, 1],
        [1, 1, 1],
    ],
];

pub fn generate_number_icon(number: usize) -> Option<Image<'static>> {
    let width = 32;
    let height = 32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];

    // Background: Transparent or Dark Circle?
    // Let's do a dark circle background for visibility
    let center_x = width as i32 / 2;
    let center_y = height as i32 / 2;
    let radius = 14;

    for y in 0..height {
        for x in 0..width {
            let dx = x as i32 - center_x;
            let dy = y as i32 - center_y;
            if dx * dx + dy * dy <= radius * radius {
                let idx = ((y * width + x) * 4) as usize;
                // Dark gray background
                rgba[idx] = 30;
                rgba[idx + 1] = 30;
                rgba[idx + 2] = 30;
                rgba[idx + 3] = 255;
            }
        }
    }

    // Draw Number
    // Simple scaling: 2x pixel size
    let scale = 2;
    let digit = if number > 9 { 9 } else { number }; // Cap at 9 for now, 0 is invalid usually but handled
    if digit > 0 && digit <= 9 {
        let pattern = &DIGITS[digit];
        let start_x = (width - (3 * scale)) / 2;
        let start_y = (height - (5 * scale)) / 2;

        for (r, row) in pattern.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == 1 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let px = start_x + (c as u32 * scale) + dx;
                            let py = start_y + (r as u32 * scale) + dy;

                            let idx = ((py * width + px) * 4) as usize;
                            // White text
                            rgba[idx] = 255;
                            rgba[idx + 1] = 255;
                            rgba[idx + 2] = 255;
                            rgba[idx + 3] = 255;
                        }
                    }
                }
            }
        }
    }

    // If number > 9, maybe draw a 'plus' dot?
    if number > 9 {
        // Draw a simple '+' at bottom right?
        // Omitted for simplicity
    }

    Some(Image::new_owned(rgba, width, height))
}
