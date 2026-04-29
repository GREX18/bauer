from PIL import Image
import os

input_folder = "input_images"   # folder with your JPGs
output_folder = "output_images" # where PNGs will go

os.makedirs(output_folder, exist_ok=True)

for filename in os.listdir(input_folder):
    if filename.lower().endswith((".jpg", ".jpeg")):
        input_path = os.path.join(input_folder, filename)
        output_path = os.path.join(
            output_folder,
            os.path.splitext(filename)[0] + ".png"
        )

        try:
            with Image.open(input_path) as img:
                # Convert to RGB to avoid issues with weird formats
                img = img.convert("RGB")
                img.save(output_path, "PNG")

            print(f"Converted: {filename}")

        except Exception as e:
            print(f"Failed: {filename} -> {e}")