# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpdS7ryu/w_I_G2.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
