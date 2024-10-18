def process_packet_info_sorted(src_dir):
    # First, collect all the c2s and s2c paths
    paths_to_process = []

    for root, dirs, files in os.walk(src_dir):
        if 'packet_info' in files:
            paths_to_process.append(root)

    # Sort the paths alphabetically
    paths_to_process.sort()

    # Now process each collected path
    for path in paths_to_process:
        packet_info_path = os.path.join(path, 'packet_info')

        # Read the packet_info file
        with open(packet_info_path, 'r') as f:
            content = f.read().strip()

        # Determine the type of content
        packet_id_match = re.match(r'packet_id=(\d+)', content)
        reexport_match = re.match(r'reexport=(\S+)', content)

        if packet_id_match:
            # Case 1: packet_id=<ID>
            packet_id_value = packet_id_match.group(1)
            packet_id_file = os.path.join(path, 'packet_id')

            # Create packet_id file
            with open(packet_id_file, 'w') as f:
                f.write(packet_id_value)

        elif reexport_match:
            # Case 2: reexport=<VERSION>
            version = reexport_match.group(1)
            version_dir = os.path.join(path, '..', version)  # move one level up and access <VERSION>

            # Find the packet_id file in the re-exported version directory
            reexport_packet_id_path = os.path.join(version_dir, 'packet_id')

            with open(reexport_packet_id_path, 'r') as f:
                reexport_packet_id_value = f.read().strip()

            # Create new files for reexport
            packet_reexport_file = os.path.join(path, 'packet_reexport')
            packet_id_file = os.path.join(path, 'packet_id')

            with open(packet_reexport_file, 'w') as f:
                f.write(version)

            with open(packet_id_file, 'w') as f:
                f.write(reexport_packet_id_value)

        # Delete the original packet_info file
        os.remove(packet_info_path)

# Call the updated function with the path to the 'src' directory
process_packet_info_sorted("src/")
