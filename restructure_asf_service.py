#!/usr/bin/env python3
"""
Restructure asf_service.rs to enable ASF finality integration with PPFA block production.

Problem:
- finality_gadget is created at line 1590
- PPFA proposer spawns at line 696
- PPFA needs finality_gadget to call propose_block() after block import (line 1013)
- Code ordering prevents this

Solution:
1. Move finality gadget creation (lines 1146-1849) to before line 648
2. Update line 694 to use the finality_gadget (already references it)
3. Uncomment finality integration code (lines 1009-1031)
"""

import sys

def restructure_file(filepath):
    print(f"Reading {filepath}...")
    with open(filepath, 'r') as f:
        lines = f.readlines()

    print(f"Total lines: {len(lines)}")

    # Line numbers are 1-indexed in the file, but 0-indexed in the list
    # So line 1146 is index 1145, line 1849 is index 1848

    # Extract the finality gadget section (lines 1142-1849, 0-indexed: 1141-1848)
    # Line 1142 starts with comment, line 1849 ends with closing brace
    finality_section_start = 1141  # Line 1142 (1-indexed)
    finality_section_end = 1848    # Line 1849 (1-indexed)

    print(f"Extracting finality gadget section (lines {finality_section_start+1}-{finality_section_end+1})...")
    finality_section = lines[finality_section_start:finality_section_end+1]

    # Verify we extracted the right section
    if not finality_section[0].strip().startswith('//'):
        print("WARNING: First line of finality section doesn't start with comment")

    print(f"Extracted {len(finality_section)} lines")
    print(f"First line: {finality_section[0][:80]}")
    print(f"Last line: {finality_section[-1][:80]}")

    # Remove the finality section from its current location
    print("Removing finality section from original location...")
    lines_without_finality = lines[:finality_section_start] + lines[finality_section_end+1:]

    # Insert point is before line 648 (0-indexed: 647)
    # This is after the RPC setup and before ASF BLOCK PRODUCTION
    insert_point = 647  # Line 648 (1-indexed)

    print(f"Inserting finality section before line {insert_point+1}...")
    print(f"Context at insert point: {lines_without_finality[insert_point][:80]}")

    # Wrap finality section in Option<> variable declaration
    finality_wrapper_start = [
        "\n",
        "    // ═══════════════════════════════════════════════════════════════════════════\n",
        "    // ASF FINALITY GADGET (Pure ASF, v108) - MOVED BEFORE PPFA\n",
        "    // ═══════════════════════════════════════════════════════════════════════════\n",
        "    // The finality_gadget must be created before PPFA spawns because PPFA needs\n",
        "    // to call propose_block() after block import (line 1013).\n",
        "\n",
        "    let finality_gadget = if asf_params.enable_finality_gadget {\n",
    ]

    # Modify the finality section to remove its own "if asf_params.enable_finality_gadget {" check
    # and add Some() return
    modified_finality_section = []
    skip_first_if = True
    brace_depth = 0

    for line in finality_section[4:]:  # Skip first 4 lines (comment header and if check)
        # Track brace depth to find the matching closing brace
        if skip_first_if and line.strip().startswith('if asf_params.enable_finality_gadget'):
            skip_first_if = False
            continue
        if skip_first_if and line.strip() == '{':
            skip_first_if = False
            continue

        modified_finality_section.append(line)

    # Remove the last closing brace and add Some(finality_gadget)
    # Find the last non-empty line
    while modified_finality_section and modified_finality_section[-1].strip() == '':
        modified_finality_section.pop()

    # Remove last closing brace
    if modified_finality_section[-1].strip() == '}':
        modified_finality_section.pop()

    finality_wrapper_end = [
        "\n",
        "        Some(finality_gadget)\n",
        "    } else {\n",
        "        log::info!(\"⚠️  ASF Finality Gadget disabled in config\");\n",
        "        None\n",
        "    };\n",
        "\n",
    ]

    # Assemble the new file
    new_lines = (
        lines_without_finality[:insert_point] +
        finality_wrapper_start +
        modified_finality_section +
        finality_wrapper_end +
        lines_without_finality[insert_point:]
    )

    print(f"New file will have {len(new_lines)} lines")

    # Now uncomment the finality integration code
    # The original lines 1009-1031 are now at a different location
    # We need to find them by searching for the comment pattern
    print("Uncommenting finality integration code...")

    uncommented_lines = []
    in_commented_block = False
    for i, line in enumerate(new_lines):
        if 'TODO: Re-enable finality integration after fixing code structure' in line:
            in_commented_block = True
            # Keep the line but modify it
            uncommented_lines.append(line.replace('TODO: Re-enable', 'ENABLED:'))
        elif in_commented_block:
            if '// }' in line and 'FINALITY INTEGRATION' not in line:
                # Uncomment the line
                uncommented_lines.append(line.replace('// ', '', 1))
            elif line.strip().startswith('// ') and '//' in line[4:10]:
                # This is a double-commented line like "//                                            // }"
                # Remove only the first //
                uncommented_lines.append(line.replace('// ', '', 1))
            elif 'Currently commented out' in line:
                # Skip this line
                continue
            elif line.strip() and not line.strip().startswith('//'):
                # End of commented block
                in_commented_block = False
                uncommented_lines.append(line)
            else:
                uncommented_lines.append(line)
        else:
            uncommented_lines.append(line)

    # Update line 694 reference
    # Find line with "let ppfa_finality_gadget = finality_gadget.clone();"
    for i, line in enumerate(uncommented_lines):
        if 'let ppfa_finality_gadget = finality_gadget.clone();' in line:
            # This line needs to handle Option<>
            print(f"Found ppfa_finality_gadget at line {i+1}")
            # Replace with proper Option handling
            uncommented_lines[i] = line.replace(
                'let ppfa_finality_gadget = finality_gadget.clone();',
                'let ppfa_finality_gadget = finality_gadget.clone(); // Now an Option<Arc<Mutex<FinalityGadget>>>'
            )
            break

    # Write the new file
    output_path = filepath
    backup_path = filepath + '.backup'

    print(f"Creating backup at {backup_path}...")
    with open(backup_path, 'w') as f:
        f.writelines(lines)

    print(f"Writing restructured file to {output_path}...")
    with open(output_path, 'w') as f:
        f.writelines(uncommented_lines)

    print("Done! File has been restructured.")
    print(f"Original file backed up to {backup_path}")
    print(f"\nSummary:")
    print(f"  - Original lines: {len(lines)}")
    print(f"  - New lines: {len(uncommented_lines)}")
    print(f"  - Finality section moved from lines 1142-1849 to before line 648")
    print(f"  - Finality integration code uncommented")

    return True

if __name__ == '__main__':
    filepath = '/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs'
    try:
        restructure_file(filepath)
        sys.exit(0)
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)
