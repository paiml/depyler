# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpVr1Boo/YY_bcR8nZZ__THu.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
