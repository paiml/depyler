# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpnL33UH/he_OrA18tQ_q1WG4_vYyi.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
