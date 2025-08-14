# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpRSpmTO/r0SvfWo_tOlu6n.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
