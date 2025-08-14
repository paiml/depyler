# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpPYI7jI/FN9_h_vylA_KE00q_MU1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
