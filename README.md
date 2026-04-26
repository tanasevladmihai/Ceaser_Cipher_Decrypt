# Ceaser_Cipher_Decrypt - a Caeser Cipher File Decryptor

This repository stores the second project for the WUT Security and Cryptography course.
This tool is designed to crypt analyze and decrypt files encrypted with a Caeser cipher. It employs a brute force method across all 256 possible shifts and validates candidates by checking for the file signatures (the magic bytes) mentioned in the project description (check [INSTRUCTIONS.md](INSTRUCTIONS.md)).

## How It Works

Instead of decrypting the entire file for every possible shift, the program efficiently checks the bytes at specific offsets to see if they match common file headers (like `PNG`, `PDF`, `RIFF/AVI`). When a match is found, it then decrypts the full content and saves it as a candidate file.

## Usage Steps

1. **Prepare the File:** Ensure the encrypted file is named `tanase_vlad` and placed in the project root (modify if different, the file I decrypted was named like that so I hardcoded it like this).
2. **Run the Decryptor:**
   ```bash
   cargo run --release
   ```
3. **Check Results:** Review the console output for detected "MATCH" entries. The program will generate files like `output_shift_x.type` for each valid candidate, where `x` is the Ceasar byte shuft value used for the decrypted file and `type` is just the file type.

> For the sample tanase_vlad file, the decrypted output is `output_shift_089.avi` where `089` is the Ceasar byte shift value that validated the RIFF signature when subtracted. Because AVI is the only RIFF signature file type in the project description, it was safe to assume encountering it will definetly mean I only have to consider saving it as `.avi`.
> In the end, the file contains a low-quality, 30 seconds long playable video of planet earth spinning.

## Verifying Results

If the program finds a match but you want to confirm the file's integrity or type, you can use **binwalk** (on Linux, macOS or via WSL):

```bash
binwalk output_shift_x.type
```

Here `binwalk` will scan the file for signatures and confirm if the internal data structures match the identified extension.

## Supported Formats

The tool only recognizes the following signatures as these were specified in the university project description:

- **Images:** PNG, JPG, BMP
- **Documents:** PDF, ZIP (Office Open XML), OLE (Legacy Office)
- **Media:** MP3, AVI, MP4
- **System:** EXE, GZ/TAR.GZ
