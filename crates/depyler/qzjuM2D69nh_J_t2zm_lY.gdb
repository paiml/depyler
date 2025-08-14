# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpmjDBl2/qzjuM2D69nh_J_t2zm_lY.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
