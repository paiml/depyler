# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpjvTIh0/bce0_3ydRc_6GWX_B6i_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
