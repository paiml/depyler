# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpmHnii1/TH56A2_mg.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
