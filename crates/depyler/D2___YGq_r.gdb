# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpdleOWc/D2___YGq_r.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
