# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpALWhye/R_R_T.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
