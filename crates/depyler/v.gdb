# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpcRntoR/v.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
