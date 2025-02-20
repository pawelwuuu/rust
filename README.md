# Image Optimization and Modification Tool

**Description:**

This program provides an easy way to optimize and modify images, offering a wide range of functions such as resizing, grayscale conversion, metadata removal, and duplicate search. The program supports popular image file formats and is easy to use thanks to its intuitive user interface.

**Features:**

* **Image Resizing:**
    * Ability to reduce the image to a specified size in megabytes (MB) or pixels (width x height).
    * MB size optimization works best for JPG and PNG formats.
    * If it is not possible to achieve the desired size, the program reduces the image to the smallest possible size, trying to maintain as much quality as possible.
* **Grayscale Conversion:**
    * Converts the image to grayscale (Luma8 format).
    * Function not available for GIF format.
* **Metadata Removal:**
    * Ability to remove metadata from selected files.
* **Duplicate Search:**
    * Function to search for duplicate images in the selected folder.
* **Duplicate Removal:**
    * Ability to remove found duplicate images.

**Supported Formats:**

GIF, JPEG, JPG, PNG, BMP, TIF, TIFF

**How to Use:**

1. Run the program.
2. Select the folder where the images are located.
3. Select the images on which you want to perform operations.
4. Select the desired option from the menu.

**Notes:**

* The program works on files located in the selected folder, which means that the original files are modified. It is recommended to work on copies of images.
* The "img" folder contains sample images for testing.

**Running the Program:**

To run the program, use the command:
cargo run

**Additional Information:**

* The program was written in Rust.
* In case of problems or questions, please contact us.

We hope that this documentation is clear and helpful. If you have any further questions, please do not hesitate to ask.
