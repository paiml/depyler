# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpmbWorx/B_vb__4X8Zf_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
