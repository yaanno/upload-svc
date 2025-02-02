import json
import zipfile
import os
import io


def create_test_archive_from_existing_file(input_file, num_files=10):
    """
    Create a ZIP archive with multiple copies of the existing large JSON file.

    Args:
        input_file (str): Path to the existing large JSON file
        num_files (int): Number of copies to include in the ZIP archive
    """
    # Ensure the output is in the svc-rust directory
    output_zip_path = "./ArchiveLarge.zip"

    # Create directory if it does not exist
    os.makedirs(os.path.dirname(output_zip_path), exist_ok=True)

    # If input file is empty or doesn't exist, create a sample JSON
    if not os.path.exists(input_file) or os.path.getsize(input_file) == 0:
        original_data = {
            "github_actions": {
                "workflow_name": "Sample Workflow",
                "repository": "example/repo",
                "status": "success",
            }
        }
    else:
        # Read the existing JSON file
        with open(input_file, "r") as f:
            original_data = json.load(f)

    # Create ZIP archive
    with zipfile.ZipFile(output_zip_path, "w", zipfile.ZIP_DEFLATED) as zipf:
        for i in range(num_files):
            filename = f"github_actions_{i}.json"

            # Write JSON data directly to the ZIP file
            json_data = json.dumps(
                original_data, indent=2, skipkeys=True, ensure_ascii=False
            ).encode("utf-8")
            zipf.writestr(filename, json_data)

    print(f"Test archive created successfully at {output_zip_path}")

    # Verify the ZIP file
    with zipfile.ZipFile(output_zip_path, "r") as zipf:
        print("ZIP file contents:")
        zipf.printdir()

        # Attempt to read and parse a file from the ZIP
        with zipf.open(zipf.namelist()[0]) as f:
            test_data = json.load(f)
            print(f"Successfully read test data from {zipf.namelist()[0]}")


if __name__ == "__main__":
    input_file = "/Users/A200246910/workspace/service-upload/large-file.json"
    create_test_archive_from_existing_file(input_file)
