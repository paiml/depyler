# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpcmtc2n/KSH82.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
