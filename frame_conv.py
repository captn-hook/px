import os
from PIL import Image
import re
from collections import defaultdict
import math

# Configuration
in_folder = "./frames/"
out_folder = "./frames_out/"

# Ensure output folder exists
os.makedirs(out_folder, exist_ok=True)

# Get all PNG files
files = [f for f in os.listdir(in_folder) if f.endswith('.png')]

# Group files by prefix (everything before the last underscore)
grouped = defaultdict(list)
pattern = re.compile(r"(.+?)_\d{4}\.png")

for file in files:
    match = pattern.match(file)
    if match:
        prefix = match.group(1)
        grouped[prefix].append(file)

def calculate_grid(n):
    best_cols = None
    best_rows = None
    best_empty = None
    best_diff = None

    for cols in range(1, n + 1):
        rows = math.ceil(n / cols)
        total_cells = cols * rows
        empty = total_cells - n
        diff = abs(cols - rows)

        if (best_empty is None or
            empty < best_empty or
            (empty == best_empty and diff < best_diff)):
            best_cols = cols
            best_rows = rows
            best_empty = empty
            best_diff = diff

    return best_cols, best_rows

# Process each group
for prefix, file_list in grouped.items():
    file_list.sort()
    images = [Image.open(os.path.join(in_folder, f)) for f in file_list]
    if not images:
        continue

    frame_width, frame_height = images[0].size
    num_frames = len(images)

    # Auto-calculate grid size
    columns, rows = calculate_grid(num_frames)

    sheet_width = columns * frame_width
    sheet_height = rows * frame_height
    sheet_image = Image.new("RGBA", (sheet_width, sheet_height), (0, 0, 0, 0))

    for index, img in enumerate(images):
        x = (index % columns) * frame_width
        y = (index // columns) * frame_height
        sheet_image.paste(img, (x, y))

    out_path = os.path.join(out_folder, f"{prefix}_{columns}x{rows}.png")
    sheet_image.save(out_path)
    print(f"✅ Saved sprite sheet: {out_path}")