# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp58Qo0M/P_b_4G_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
