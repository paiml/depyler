# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpaY5Q3N/MWIle4_UG1E_HG9_6.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
