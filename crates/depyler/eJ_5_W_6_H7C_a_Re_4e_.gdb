# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpORh6RN/eJ_5_W_6_H7C_a_Re_4e_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
