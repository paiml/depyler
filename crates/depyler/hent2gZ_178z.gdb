# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpN1Wtgr/hent2gZ_178z.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
