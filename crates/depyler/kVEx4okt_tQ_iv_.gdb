# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpSPsnn3/kVEx4okt_tQ_iv_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
