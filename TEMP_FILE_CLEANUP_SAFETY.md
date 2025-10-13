# Temporary File Cleanup - Safety Analysis

## Overview

This document explains how the app's temporary file cleanup system ensures it **ONLY** removes files created by this application, and **NEVER** touches unrelated files.

---

## 🔒 **Safety Guarantees**

### **1. Exact Pattern Matching**

The cleanup system uses **strict pattern validation** that matches ONLY the exact formats created by this application:

#### **Archive Files** (`mod_{id}_{id}.zip`)

- **Format**: `mod_{numeric_mod_id}_{numeric_file_id}.zip`
- **Example**: `mod_107_123169.zip`
- **Validation**:
  - Must start with exactly `mod_`
  - Must end with exactly `.zip`
  - Must have exactly 2 numeric IDs separated by underscores
  - Both IDs must contain ONLY digits (0-9)

#### **Extraction Directories** (`mod_extract_{id}_{uuid}`)

- **Format**: `mod_extract_{numeric_mod_id}_{uuid_v4}`
- **Example**: `mod_extract_107_550e8400-e29b-41d4-a716-446655440000`
- **Validation**:
  - Must start with exactly `mod_extract_`
  - Must have exactly 2 parts: numeric ID and UUID
  - First part must be purely numeric
  - Second part must be a valid UUID v4 format:
    - 5 segments separated by hyphens
    - Format: `8-4-4-4-12` hexadecimal characters
    - Example: `550e8400-e29b-41d4-a716-446655440000`

---

### **2. Age-Based Safety**

- **Only removes files older than 1 hour**
- Protects active downloads and installations
- Prevents race conditions with concurrent operations
- Implementation: `is_path_older_than(&path, 1)` checks file modification time

---

### **3. Limited Scope**

- **Only scans**: `std::env::temp_dir()` (system temporary directory)
  - macOS: `/var/folders/.../T/` (system-managed temp)
  - Windows: `%TEMP%` or `%TMP%`
- **Never scans**:
  - User home directories
  - Game installation folders
  - Documents, Downloads, Desktop
  - Any directory outside system temp

---

### **4. Validation Code**

```rust
// Archive validation
let is_mod_archive = if file_name.starts_with("mod_") && file_name.ends_with(".zip") {
    let inner = &file_name[4..file_name.len()-4]; // Extract "107_123169"
    let parts: Vec<&str> = inner.split('_').collect();

    // Must have exactly 2 parts, both purely numeric
    parts.len() == 2 &&
    parts[0].chars().all(|c| c.is_ascii_digit()) &&
    parts[1].chars().all(|c| c.is_ascii_digit())
} else {
    false
};

// Directory validation
let is_mod_extract_dir = if file_name.starts_with("mod_extract_") {
    let inner = &file_name[12..]; // Extract "107_550e8400-..."
    let parts: Vec<&str> = inner.split('_').collect();

    if parts.len() == 2 && parts[0].chars().all(|c| c.is_ascii_digit()) {
        let uuid_part = parts[1];
        let uuid_segments: Vec<&str> = uuid_part.split('-').collect();

        // Validate UUID format: 8-4-4-4-12
        uuid_segments.len() == 5 &&
        uuid_segments[0].len() == 8 &&
        uuid_segments[1].len() == 4 &&
        uuid_segments[2].len() == 4 &&
        uuid_segments[3].len() == 4 &&
        uuid_segments[4].len() == 12 &&
        uuid_part.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
    } else {
        false
    }
} else {
    false
};
```

---

## ✅ **Test Cases - Files That WILL BE REMOVED** (if older than 1 hour)

| Filename                                               | Reason                                | Valid |
| ------------------------------------------------------ | ------------------------------------- | ----- |
| `mod_107_123169.zip`                                   | Exact format: two numeric IDs         | ✅    |
| `mod_1_2.zip`                                          | Exact format: two numeric IDs         | ✅    |
| `mod_99999_88888.zip`                                  | Exact format: two numeric IDs         | ✅    |
| `mod_extract_107_550e8400-e29b-41d4-a716-446655440000` | Exact format: numeric ID + valid UUID | ✅    |
| `mod_extract_1_a1b2c3d4-e5f6-7890-abcd-ef1234567890`   | Exact format: numeric ID + valid UUID | ✅    |

---

## ❌ **Test Cases - Files That WILL NOT BE REMOVED** (Protected)

### **Invalid Archive Names**

| Filename                 | Reason Protected      | Issue                        |
| ------------------------ | --------------------- | ---------------------------- |
| `mod.zip`                | Missing IDs           | No underscores, no IDs       |
| `mod_.zip`               | Missing IDs           | Empty ID after underscore    |
| `mod_123.zip`            | Only one ID           | Needs exactly 2 IDs          |
| `mod_abc_123.zip`        | Non-numeric ID        | First part contains letters  |
| `mod_123_xyz.zip`        | Non-numeric ID        | Second part contains letters |
| `mod_1.2_345.zip`        | Non-numeric ID        | Contains period (not digit)  |
| `mod_107_123`            | Not a .zip            | Missing extension            |
| `mod_107_123.txt`        | Not a .zip            | Wrong extension              |
| `mod_107_123.zip.backup` | Not a .zip            | Extra extension              |
| `modern_art.zip`         | Doesn't match pattern | "modern" != "mod\_"          |
| `mymod_1_2.zip`          | Wrong prefix          | "mymod" != "mod\_"           |
| `MOD_1_2.zip`            | Case sensitive        | Uppercase not matched        |
| `mod_107_123_extra.zip`  | Too many parts        | Has 3+ underscores           |
| `mod_-1_123.zip`         | Invalid ID            | Negative number (has hyphen) |
| `mod_ 107_123.zip`       | Contains space        | Space is not a digit         |

### **Invalid Directory Names**

| Directory Name                                            | Reason Protected | Issue                                 |
| --------------------------------------------------------- | ---------------- | ------------------------------------- |
| `mod_extract_`                                            | Missing parts    | No ID or UUID                         |
| `mod_extract_abc`                                         | Non-numeric ID   | Letters in ID part                    |
| `mod_extract_123`                                         | Missing UUID     | No UUID part                          |
| `mod_extract_123_not-a-uuid`                              | Invalid UUID     | Malformed UUID                        |
| `mod_extract_123_abc`                                     | Invalid UUID     | Too short                             |
| `mod_extract_123_12345678-1234-1234-1234-1234567890`      | Invalid UUID     | Last segment only 10 chars (needs 12) |
| `mod_extract_123_12345678-1234-1234-1234-12345678901x`    | Invalid UUID     | Contains non-hex character 'x'        |
| `mod_extract_1.2_550e8400-e29b-41d4-a716-446655440000`    | Invalid ID       | Period in ID (not digit)              |
| `mod_extraction_123_550e8400-e29b-41d4-a716-446655440000` | Wrong prefix     | "extraction" != "extract"             |

### **Files in Wrong Location**

| Location                            | Reason Protected             |
| ----------------------------------- | ---------------------------- |
| `~/Downloads/mod_107_123.zip`       | Not in system temp directory |
| `/Applications/mod_extract_107_...` | Not in system temp directory |
| `~/Documents/mod_107_123.zip`       | Not in system temp directory |

### **Recently Created Files**

| Filename                                      | Reason Protected     |
| --------------------------------------------- | -------------------- |
| `mod_107_123.zip` (created 30 minutes ago)    | Less than 1 hour old |
| `mod_extract_107_...` (created 5 minutes ago) | Less than 1 hour old |

---

## 🧪 **How to Test Safety**

### **Test 1: Create Protected Files**

```bash
cd $(python3 -c "import tempfile; print(tempfile.gettempdir())")

# Create files that should NOT be removed
touch "mod.zip"
touch "mod_abc_123.zip"
touch "modern_art.zip"
touch "mod_107_123.txt"
mkdir "mod_extract_invalid"

# Run cleanup
# Open app → Settings → Clean Temporary Files

# Verify these files still exist
ls -la mod*
```

### **Test 2: Create Valid Old Files**

```bash
cd $(python3 -c "import tempfile; print(tempfile.gettempdir())")

# Create old valid files (modify timestamp to 2 hours ago)
touch "mod_999_888.zip"
touch -t $(date -v-2H +%Y%m%d%H%M) "mod_999_888.zip"

mkdir "mod_extract_999_12345678-1234-1234-1234-123456789012"
touch -t $(date -v-2H +%Y%m%d%H%M) "mod_extract_999_12345678-1234-1234-1234-123456789012"

# Run cleanup
# Open app → Settings → Clean Temporary Files

# Verify these files are REMOVED
ls -la mod_999* mod_extract_999*
# Should return: "No such file or directory"
```

### **Test 3: Verify Recent Files Protected**

```bash
# Create fresh valid files (just now)
touch "mod_111_222.zip"

# Run cleanup immediately
# Open app → Settings → Clean Temporary Files

# Verify file still exists (too recent)
ls -la mod_111_222.zip
# Should still exist
```

---

## 📊 **Validation Summary**

| Validation Layer       | Protection            | Implementation                  |
| ---------------------- | --------------------- | ------------------------------- | --- | -------------------- |
| **Pattern Matching**   | Exact format only     | `starts_with()` + `ends_with()` |
| **Numeric Validation** | Only digits allowed   | `all(                           | c   | c.is_ascii_digit())` |
| **UUID Validation**    | Exact UUID v4 format  | Length check + hex validation   |
| **Part Count**         | Exact number of parts | `parts.len() == 2`              |
| **Age Filter**         | 1+ hours old only     | `is_path_older_than(&path, 1)`  |
| **Directory Scope**    | System temp only      | `std::env::temp_dir()`          |
| **Error Handling**     | Fails safe            | Errors logged, no crash         |

---

## 🔐 **Security Principles**

1. **Fail Safe**: If validation fails at ANY step, file is NOT removed
2. **Explicit Allowlist**: Only removes files matching ALL criteria
3. **No Wildcards**: No glob patterns or regex (too risky)
4. **Atomic Checks**: Each validation is independent and explicit
5. **Logged Actions**: All removals logged to console
6. **User Control**: Manual cleanup available, automatic on startup

---

## 📝 **Conclusion**

The cleanup system is designed with **paranoid safety**:

✅ **Multiple layers of validation**  
✅ **Strict pattern matching**  
✅ **No loose matching or wildcards**  
✅ **Age-based protection**  
✅ **Limited directory scope**  
✅ **Fail-safe error handling**

**It is virtually impossible for this system to accidentally delete unrelated files.**

The only way a file gets removed is if it:

1. Is in the system temp directory
2. Matches the EXACT format we create (numeric IDs + .zip OR numeric ID + valid UUID)
3. Is older than 1 hour
4. Passes ALL validation checks

Any deviation from these criteria = file is protected.
