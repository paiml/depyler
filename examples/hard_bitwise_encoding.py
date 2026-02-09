"""Pathological bitwise operations and encoding/decoding patterns for transpiler stress testing."""


# --- Bit Manipulation ---


def popcount(n: int) -> int:
    """Count the number of set bits in a non-negative integer."""
    count: int = 0
    val: int = n
    while val > 0:
        count = count + (val & 1)
        val = val >> 1
    return count


def parity(n: int) -> int:
    """Return 0 if even number of set bits, 1 if odd."""
    val: int = n
    val = val ^ (val >> 16)
    val = val ^ (val >> 8)
    val = val ^ (val >> 4)
    val = val ^ (val >> 2)
    val = val ^ (val >> 1)
    return val & 1


def reverse_bits_32(n: int) -> int:
    """Reverse 16 bits of an integer (fits in i32)."""
    result: int = 0
    i: int = 0
    val: int = n & 0xFFFF
    while i < 16:
        result = (result << 1) | (val & 1)
        val = val >> 1
        i = i + 1
    return result


def isolate_lowest_set_bit(n: int) -> int:
    """Isolate the lowest set bit. Returns 0 if n is 0."""
    if n == 0:
        return 0
    return n & (-n)


def clear_lowest_set_bit(n: int) -> int:
    """Clear the lowest set bit of n."""
    return n & (n - 1)


def highest_set_bit_pos(n: int) -> int:
    """Return the position of the highest set bit, or -1 if n is 0."""
    if n <= 0:
        return -1
    pos: int = 0
    val: int = n
    while val > 1:
        val = val >> 1
        pos = pos + 1
    return pos


def next_power_of_two(n: int) -> int:
    """Return the smallest power of two >= n. Assumes n > 0."""
    if n <= 1:
        return 1
    val: int = n - 1
    val = val | (val >> 1)
    val = val | (val >> 2)
    val = val | (val >> 4)
    val = val | (val >> 8)
    val = val | (val >> 16)
    return val + 1


def swap_bits(n: int, i: int, j: int) -> int:
    """Swap bits at positions i and j in n."""
    bit_i: int = (n >> i) & 1
    bit_j: int = (n >> j) & 1
    if bit_i == bit_j:
        return n
    mask: int = (1 << i) | (1 << j)
    return n ^ mask


# --- Bitfield Packing/Unpacking ---


def pack_rgb(r: int, g: int, b: int) -> int:
    """Pack three 8-bit color channels into a single 24-bit integer."""
    return ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)


def unpack_r(packed: int) -> int:
    """Extract the red channel from a packed RGB value."""
    return (packed >> 16) & 0xFF


def unpack_g(packed: int) -> int:
    """Extract the green channel from a packed RGB value."""
    return (packed >> 8) & 0xFF


def unpack_b(packed: int) -> int:
    """Extract the blue channel from a packed RGB value."""
    return packed & 0xFF


def pack_fields(a: int, b: int, c: int, d: int) -> int:
    """Pack four 8-bit fields into a 32-bit integer."""
    return ((a & 0xFF) << 24) | ((b & 0xFF) << 16) | ((c & 0xFF) << 8) | (d & 0xFF)


def extract_field(packed: int, offset: int, width: int) -> int:
    """Extract a bitfield of given width at given bit offset."""
    mask: int = (1 << width) - 1
    return (packed >> offset) & mask


def set_field(packed: int, offset: int, width: int, value: int) -> int:
    """Set a bitfield of given width at given bit offset to value."""
    mask: int = (1 << width) - 1
    cleared: int = packed & ~(mask << offset)
    return cleared | ((value & mask) << offset)


# --- XOR Cipher ---


def xor_encode(data: list[int], key: int) -> list[int]:
    """Encode a list of integers by XORing each with the key."""
    result: list[int] = []
    i: int = 0
    while i < len(data):
        result.append(data[i] ^ (key & 0xFF))
        i = i + 1
    return result


def xor_encode_rolling(data: list[int], key: int) -> list[int]:
    """Encode with a rolling XOR key that shifts after each byte."""
    result: list[int] = []
    current_key: int = key & 0xFF
    i: int = 0
    while i < len(data):
        encoded: int = data[i] ^ current_key
        result.append(encoded)
        current_key = ((current_key << 1) | (current_key >> 7)) & 0xFF
        i = i + 1
    return result


# --- Run-Length Encoding ---


def rle_encode(data: list[int]) -> list[int]:
    """Run-length encode a list into [value, count, value, count, ...] pairs."""
    if len(data) == 0:
        return []
    result: list[int] = []
    current: int = data[0]
    count: int = 1
    i: int = 1
    while i < len(data):
        if data[i] == current:
            count = count + 1
        else:
            result.append(current)
            result.append(count)
            current = data[i]
            count = 1
        i = i + 1
    result.append(current)
    result.append(count)
    return result


def rle_decode(encoded: list[int]) -> list[int]:
    """Decode a run-length encoded list back to original data."""
    result: list[int] = []
    i: int = 0
    while i < len(encoded) - 1:
        value: int = encoded[i]
        count: int = encoded[i + 1]
        j: int = 0
        while j < count:
            result.append(value)
            j = j + 1
        i = i + 2
    return result


# --- Delta Encoding ---


def delta_encode(data: list[int]) -> list[int]:
    """Delta-encode: first element as-is, then differences."""
    if len(data) == 0:
        return []
    result: list[int] = [data[0]]
    i: int = 1
    while i < len(data):
        result.append(data[i] - data[i - 1])
        i = i + 1
    return result


def delta_decode(encoded: list[int]) -> list[int]:
    """Decode a delta-encoded list back to original values."""
    if len(encoded) == 0:
        return []
    result: list[int] = [encoded[0]]
    i: int = 1
    while i < len(encoded):
        result.append(result[i - 1] + encoded[i])
        i = i + 1
    return result


# --- Variable-Length Encoding (like LEB128 / protobuf varint) ---


def varint_encode(n: int) -> list[int]:
    """Encode a non-negative integer into 7-bit chunks with high-bit continuation."""
    if n == 0:
        return [0]
    result: list[int] = []
    val: int = n
    while val > 0:
        chunk: int = val & 0x7F
        val = val >> 7
        if val > 0:
            chunk = chunk | 0x80
        result.append(chunk)
    return result


def varint_decode(encoded: list[int]) -> int:
    """Decode a varint-encoded list back to an integer."""
    result: int = 0
    shift: int = 0
    i: int = 0
    while i < len(encoded):
        chunk: int = encoded[i] & 0x7F
        result = result | (chunk << shift)
        if (encoded[i] & 0x80) == 0:
            break
        shift = shift + 7
        i = i + 1
    return result


def varint_encode_list(data: list[int]) -> list[int]:
    """Encode a list of non-negative integers as concatenated varints."""
    result: list[int] = []
    i: int = 0
    while i < len(data):
        encoded: list[int] = varint_encode(data[i])
        j: int = 0
        while j < len(encoded):
            result.append(encoded[j])
            j = j + 1
        i = i + 1
    return result


# --- Gray Code ---


def to_gray(n: int) -> int:
    """Convert a binary number to Gray code."""
    return n ^ (n >> 1)


def from_gray(gray: int) -> int:
    """Convert a Gray code back to binary."""
    n: int = gray
    mask: int = n >> 1
    while mask > 0:
        n = n ^ mask
        mask = mask >> 1
    return n


# --- Hamming Distance ---


def hamming_distance(a: int, b: int) -> int:
    """Count the number of bit positions where a and b differ."""
    return popcount(a ^ b)


# --- Bit Rotation ---


def rotate_left_32(n: int, amount: int) -> int:
    """Rotate a 16-bit value left by amount positions (fits in i32)."""
    val: int = n & 0xFFFF
    shift: int = amount & 15
    return ((val << shift) | (val >> (16 - shift))) & 0xFFFF


def rotate_right_32(n: int, amount: int) -> int:
    """Rotate a 16-bit value right by amount positions (fits in i32)."""
    val: int = n & 0xFFFF
    shift: int = amount & 15
    return ((val >> shift) | (val << (16 - shift))) & 0xFFFF


# --- CRC-like Checksum ---


def crc8_simple(data: list[int], poly: int) -> int:
    """Compute an 8-bit CRC-like checksum using polynomial division over GF(2)."""
    crc: int = 0
    i: int = 0
    while i < len(data):
        crc = crc ^ (data[i] & 0xFF)
        bit: int = 0
        while bit < 8:
            if (crc & 0x80) != 0:
                crc = ((crc << 1) ^ poly) & 0xFF
            else:
                crc = (crc << 1) & 0xFF
            bit = bit + 1
        i = i + 1
    return crc


def crc16_simple(data: list[int], poly: int) -> int:
    """Compute a 16-bit CRC-like checksum."""
    crc: int = 0xFFFF
    i: int = 0
    while i < len(data):
        crc = crc ^ (data[i] & 0xFF)
        bit: int = 0
        while bit < 8:
            if (crc & 1) != 0:
                crc = (crc >> 1) ^ poly
            else:
                crc = crc >> 1
            bit = bit + 1
        i = i + 1
    return crc & 0xFFFF


# --- Base64-like Encoding ---


def base64_like_encode(data: list[int]) -> list[int]:
    """Convert list of 8-bit values to list of 6-bit values (base64-style grouping)."""
    result: list[int] = []
    buffer: int = 0
    bits_in_buffer: int = 0
    i: int = 0
    while i < len(data):
        buffer = (buffer << 8) | (data[i] & 0xFF)
        bits_in_buffer = bits_in_buffer + 8
        while bits_in_buffer >= 6:
            bits_in_buffer = bits_in_buffer - 6
            result.append((buffer >> bits_in_buffer) & 0x3F)
        i = i + 1
    if bits_in_buffer > 0:
        result.append((buffer << (6 - bits_in_buffer)) & 0x3F)
    return result


def base64_like_decode(encoded: list[int], original_len: int) -> list[int]:
    """Convert list of 6-bit values back to 8-bit values."""
    result: list[int] = []
    buffer: int = 0
    bits_in_buffer: int = 0
    i: int = 0
    while i < len(encoded):
        buffer = (buffer << 6) | (encoded[i] & 0x3F)
        bits_in_buffer = bits_in_buffer + 6
        while bits_in_buffer >= 8:
            bits_in_buffer = bits_in_buffer - 8
            result.append((buffer >> bits_in_buffer) & 0xFF)
        i = i + 1
    while len(result) > original_len:
        result.pop()
    return result


# --- Huffman-like Frequency Counting ---


def frequency_table(data: list[int]) -> dict[int, int]:
    """Build a frequency table mapping values to their counts."""
    table: dict[int, int] = {}
    i: int = 0
    while i < len(data):
        val: int = data[i]
        if val in table:
            table[val] = table[val] + 1
        else:
            table[val] = 1
        i = i + 1
    return table


def assign_bit_lengths(freq: dict[int, int]) -> dict[int, int]:
    """Assign bit lengths by frequency rank: most frequent gets 1 bit, next 2, etc."""
    if len(freq) == 0:
        return {}
    sorted_keys: list[int] = sorted(freq.keys(), key=lambda k: freq[k], reverse=True)
    lengths: dict[int, int] = {}
    rank: int = 1
    i: int = 0
    while i < len(sorted_keys):
        bit_len: int = highest_set_bit_pos(rank) + 1
        if bit_len < 1:
            bit_len = 1
        lengths[sorted_keys[i]] = bit_len
        rank = rank + 1
        i = i + 1
    return lengths


def total_encoded_bits(data: list[int], bit_lengths: dict[int, int]) -> int:
    """Compute total bits needed to encode data with given bit lengths."""
    total: int = 0
    i: int = 0
    while i < len(data):
        if data[i] in bit_lengths:
            total = total + bit_lengths[data[i]]
        else:
            total = total + 8
        i = i + 1
    return total


# --- Zigzag Encoding (protobuf style) ---


def zigzag_encode(n: int) -> int:
    """Map signed integer to unsigned: 0->0, -1->1, 1->2, -2->3, 2->4, etc."""
    if n >= 0:
        return n * 2
    return (-n) * 2 - 1


def zigzag_decode(n: int) -> int:
    """Decode a zigzag-encoded unsigned integer back to signed."""
    if (n & 1) == 0:
        return n >> 1
    return -((n + 1) >> 1)


# --- Prefix-Free Code Validation ---


def is_prefix_of(code_a: int, len_a: int, code_b: int, len_b: int) -> int:
    """Check if code_a (with len_a bits) is a prefix of code_b (with len_b bits). Returns 1 or 0."""
    if len_a > len_b:
        return 0
    shift: int = len_b - len_a
    if (code_b >> shift) == code_a:
        return 1
    return 0


def validate_prefix_free(codes: list[int], lengths: list[int]) -> int:
    """Check if a set of codes with given bit lengths is prefix-free. Returns 1 if valid, 0 if not."""
    n: int = len(codes)
    i: int = 0
    while i < n:
        j: int = 0
        while j < n:
            if i != j:
                if is_prefix_of(codes[i], lengths[i], codes[j], lengths[j]) == 1:
                    return 0
            j = j + 1
        i = i + 1
    return 1


# --- Bit Interleaving / Morton Codes ---


def interleave_bits(x: int, y: int) -> int:
    """Interleave the lower 16 bits of x and y into a 32-bit Morton code."""
    result: int = 0
    i: int = 0
    while i < 16:
        result = result | (((x >> i) & 1) << (2 * i))
        result = result | (((y >> i) & 1) << (2 * i + 1))
        i = i + 1
    return result


def deinterleave_x(morton: int) -> int:
    """Extract the x component from a Morton code."""
    result: int = 0
    i: int = 0
    while i < 16:
        result = result | (((morton >> (2 * i)) & 1) << i)
        i = i + 1
    return result


def deinterleave_y(morton: int) -> int:
    """Extract the y component from a Morton code."""
    result: int = 0
    i: int = 0
    while i < 16:
        result = result | (((morton >> (2 * i + 1)) & 1) << i)
        i = i + 1
    return result


def morton_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute the absolute difference of Morton codes for two 2D points."""
    m1: int = interleave_bits(x1, y1)
    m2: int = interleave_bits(x2, y2)
    if m1 > m2:
        return m1 - m2
    return m2 - m1


# ========== TEST FUNCTIONS ==========


def test_popcount_and_parity() -> int:
    """Test popcount and parity on known values."""
    result: int = 0
    if popcount(0) == 0:
        result = result + 1
    if popcount(0xFF) == 8:
        result = result + 1
    if popcount(0b10101010) == 4:
        result = result + 1
    if parity(0b1111) == 0:
        result = result + 1
    if parity(0b111) == 1:
        result = result + 1
    if parity(0) == 0:
        result = result + 1
    return result


def test_reverse_bits() -> int:
    """Test 16-bit bit reversal."""
    result: int = 0
    if reverse_bits_32(0) == 0:
        result = result + 1
    if reverse_bits_32(1) == 32768:
        result = result + 1
    rev: int = reverse_bits_32(32768)
    if rev == 1:
        result = result + 1
    if reverse_bits_32(reverse_bits_32(12345)) == 12345:
        result = result + 1
    return result


def test_isolate_and_clear() -> int:
    """Test isolate lowest set bit and clear lowest set bit."""
    result: int = 0
    if isolate_lowest_set_bit(12) == 4:
        result = result + 1
    if isolate_lowest_set_bit(0) == 0:
        result = result + 1
    if isolate_lowest_set_bit(8) == 8:
        result = result + 1
    if clear_lowest_set_bit(12) == 8:
        result = result + 1
    if clear_lowest_set_bit(8) == 0:
        result = result + 1
    if clear_lowest_set_bit(0) == -1:
        result = result + 1
    return result


def test_bit_utilities() -> int:
    """Test highest_set_bit_pos, next_power_of_two, swap_bits."""
    result: int = 0
    if highest_set_bit_pos(1) == 0:
        result = result + 1
    if highest_set_bit_pos(8) == 3:
        result = result + 1
    if highest_set_bit_pos(0) == -1:
        result = result + 1
    if next_power_of_two(5) == 8:
        result = result + 1
    if next_power_of_two(8) == 8:
        result = result + 1
    if next_power_of_two(1) == 1:
        result = result + 1
    swapped: int = swap_bits(0b1010, 1, 2)
    if swapped == 0b1100:
        result = result + 1
    return result


def test_pack_unpack_rgb() -> int:
    """Test RGB packing and unpacking."""
    result: int = 0
    packed: int = pack_rgb(255, 128, 0)
    if unpack_r(packed) == 255:
        result = result + 1
    if unpack_g(packed) == 128:
        result = result + 1
    if unpack_b(packed) == 0:
        result = result + 1
    packed2: int = pack_rgb(0, 0, 0)
    if packed2 == 0:
        result = result + 1
    return result


def test_bitfield_ops() -> int:
    """Test pack_fields, extract_field, set_field."""
    result: int = 0
    packed: int = pack_fields(0xAB, 0xCD, 0xEF, 0x12)
    if extract_field(packed, 24, 8) == 0xAB:
        result = result + 1
    if extract_field(packed, 16, 8) == 0xCD:
        result = result + 1
    if extract_field(packed, 8, 8) == 0xEF:
        result = result + 1
    if extract_field(packed, 0, 8) == 0x12:
        result = result + 1
    modified: int = set_field(packed, 8, 8, 0x99)
    if extract_field(modified, 8, 8) == 0x99:
        result = result + 1
    return result


def test_xor_cipher() -> int:
    """Test XOR encode is reversible."""
    result: int = 0
    data: list[int] = [72, 101, 108, 108, 111]
    key: int = 42
    encoded: list[int] = xor_encode(data, key)
    decoded: list[int] = xor_encode(encoded, key)
    if len(decoded) == len(data):
        result = result + 1
    match: int = 1
    i: int = 0
    while i < len(data):
        if decoded[i] != data[i]:
            match = 0
        i = i + 1
    result = result + match
    rolling_enc: list[int] = xor_encode_rolling(data, key)
    if len(rolling_enc) == len(data):
        result = result + 1
    return result


def test_rle() -> int:
    """Test run-length encoding roundtrip."""
    result: int = 0
    data: list[int] = [1, 1, 1, 2, 2, 3, 3, 3, 3]
    encoded: list[int] = rle_encode(data)
    if encoded[0] == 1:
        result = result + 1
    if encoded[1] == 3:
        result = result + 1
    if encoded[2] == 2:
        result = result + 1
    if encoded[3] == 2:
        result = result + 1
    decoded: list[int] = rle_decode(encoded)
    if len(decoded) == len(data):
        result = result + 1
    match: int = 1
    i: int = 0
    while i < len(data):
        if decoded[i] != data[i]:
            match = 0
        i = i + 1
    result = result + match
    return result


def test_delta_encoding() -> int:
    """Test delta encoding roundtrip."""
    result: int = 0
    data: list[int] = [10, 13, 17, 20, 25]
    encoded: list[int] = delta_encode(data)
    if encoded[0] == 10:
        result = result + 1
    if encoded[1] == 3:
        result = result + 1
    if encoded[2] == 4:
        result = result + 1
    decoded: list[int] = delta_decode(encoded)
    match: int = 1
    i: int = 0
    while i < len(data):
        if decoded[i] != data[i]:
            match = 0
        i = i + 1
    result = result + match
    empty_enc: list[int] = delta_encode([])
    if len(empty_enc) == 0:
        result = result + 1
    return result


def test_varint() -> int:
    """Test variable-length integer encoding roundtrip."""
    result: int = 0
    enc0: list[int] = varint_encode(0)
    if enc0[0] == 0:
        result = result + 1
    if varint_decode(enc0) == 0:
        result = result + 1
    enc300: list[int] = varint_encode(300)
    if varint_decode(enc300) == 300:
        result = result + 1
    enc_big: list[int] = varint_encode(123456789)
    if varint_decode(enc_big) == 123456789:
        result = result + 1
    if len(enc_big) > 1:
        result = result + 1
    return result


def test_gray_code() -> int:
    """Test Gray code conversion roundtrip."""
    result: int = 0
    if to_gray(0) == 0:
        result = result + 1
    if to_gray(1) == 1:
        result = result + 1
    if to_gray(2) == 3:
        result = result + 1
    if to_gray(3) == 2:
        result = result + 1
    i: int = 0
    all_roundtrip: int = 1
    while i < 256:
        if from_gray(to_gray(i)) != i:
            all_roundtrip = 0
        i = i + 1
    result = result + all_roundtrip
    gray_diff: int = 1
    j: int = 1
    while j < 64:
        diff: int = to_gray(j) ^ to_gray(j - 1)
        if popcount(diff) != 1:
            gray_diff = 0
        j = j + 1
    result = result + gray_diff
    return result


def test_hamming_and_rotation() -> int:
    """Test Hamming distance and bit rotation."""
    result: int = 0
    if hamming_distance(0, 0) == 0:
        result = result + 1
    if hamming_distance(0xFF, 0x00) == 8:
        result = result + 1
    if hamming_distance(0b1010, 0b0101) == 4:
        result = result + 1
    rot: int = rotate_left_32(1, 4)
    if rot == 16:
        result = result + 1
    rot_back: int = rotate_right_32(rot, 4)
    if rot_back == 1:
        result = result + 1
    full_rot: int = rotate_left_32(0x5EAD, 16)
    if full_rot == 0x5EAD:
        result = result + 1
    return result


def test_crc() -> int:
    """Test CRC checksum computation."""
    result: int = 0
    data1: list[int] = [0x01, 0x02, 0x03]
    crc1: int = crc8_simple(data1, 0x07)
    if crc1 >= 0:
        result = result + 1
    if crc1 <= 255:
        result = result + 1
    data2: list[int] = [0x01, 0x02, 0x03]
    crc2: int = crc8_simple(data2, 0x07)
    if crc1 == crc2:
        result = result + 1
    crc16_val: int = crc16_simple(data1, 0xA001)
    if crc16_val >= 0:
        result = result + 1
    if crc16_val <= 0xFFFF:
        result = result + 1
    return result


def test_base64_like() -> int:
    """Test base64-like encoding roundtrip."""
    result: int = 0
    data: list[int] = [65, 66, 67]
    encoded: list[int] = base64_like_encode(data)
    if len(encoded) == 4:
        result = result + 1
    i: int = 0
    all_valid: int = 1
    while i < len(encoded):
        if encoded[i] < 0:
            all_valid = 0
        if encoded[i] > 63:
            all_valid = 0
        i = i + 1
    result = result + all_valid
    decoded: list[int] = base64_like_decode(encoded, 3)
    if len(decoded) == 3:
        result = result + 1
    match: int = 1
    j: int = 0
    while j < len(data):
        if decoded[j] != data[j]:
            match = 0
        j = j + 1
    result = result + match
    return result


def test_zigzag() -> int:
    """Test zigzag encoding for signed-to-unsigned mapping."""
    result: int = 0
    if zigzag_encode(0) == 0:
        result = result + 1
    if zigzag_encode(-1) == 1:
        result = result + 1
    if zigzag_encode(1) == 2:
        result = result + 1
    if zigzag_encode(-2) == 3:
        result = result + 1
    if zigzag_encode(2) == 4:
        result = result + 1
    roundtrip_ok: int = 1
    n: int = -100
    while n <= 100:
        if zigzag_decode(zigzag_encode(n)) != n:
            roundtrip_ok = 0
        n = n + 1
    result = result + roundtrip_ok
    return result


def test_prefix_free() -> int:
    """Test prefix-free code validation."""
    result: int = 0
    codes_good: list[int] = [0b0, 0b10, 0b11]
    lens_good: list[int] = [1, 2, 2]
    if validate_prefix_free(codes_good, lens_good) == 1:
        result = result + 1
    codes_bad: list[int] = [0b0, 0b01, 0b1]
    lens_bad: list[int] = [1, 2, 1]
    if validate_prefix_free(codes_bad, lens_bad) == 0:
        result = result + 1
    codes_single: list[int] = [0b101]
    lens_single: list[int] = [3]
    if validate_prefix_free(codes_single, lens_single) == 1:
        result = result + 1
    return result


def test_morton_codes() -> int:
    """Test bit interleaving / Morton codes."""
    result: int = 0
    morton: int = interleave_bits(5, 3)
    if deinterleave_x(morton) == 5:
        result = result + 1
    if deinterleave_y(morton) == 3:
        result = result + 1
    if interleave_bits(0, 0) == 0:
        result = result + 1
    m1: int = interleave_bits(1, 0)
    if m1 == 1:
        result = result + 1
    m2: int = interleave_bits(0, 1)
    if m2 == 2:
        result = result + 1
    dist: int = morton_distance(0, 0, 1, 1)
    if dist > 0:
        result = result + 1
    return result


def test_huffman_like() -> int:
    """Test frequency table and bit length assignment."""
    result: int = 0
    data: list[int] = [1, 1, 1, 2, 2, 3]
    freq: dict[int, int] = frequency_table(data)
    if freq[1] == 3:
        result = result + 1
    if freq[2] == 2:
        result = result + 1
    if freq[3] == 1:
        result = result + 1
    lengths: dict[int, int] = assign_bit_lengths(freq)
    if lengths[1] <= lengths[3]:
        result = result + 1
    total: int = total_encoded_bits(data, lengths)
    if total > 0:
        result = result + 1
    return result


def run_all_tests() -> int:
    """Run all test functions and return the sum of passed checks."""
    total: int = 0
    total = total + test_popcount_and_parity()
    total = total + test_reverse_bits()
    total = total + test_isolate_and_clear()
    total = total + test_bit_utilities()
    total = total + test_pack_unpack_rgb()
    total = total + test_bitfield_ops()
    total = total + test_xor_cipher()
    total = total + test_rle()
    total = total + test_delta_encoding()
    total = total + test_varint()
    total = total + test_gray_code()
    total = total + test_hamming_and_rotation()
    total = total + test_crc()
    total = total + test_base64_like()
    total = total + test_zigzag()
    total = total + test_prefix_free()
    total = total + test_morton_codes()
    total = total + test_huffman_like()
    return total


if __name__ == "__main__":
    passed: int = run_all_tests()
    print(passed)
