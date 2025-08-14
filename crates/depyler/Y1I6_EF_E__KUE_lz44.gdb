# GDB initialization script for Depyler debugging
# Source: /tmp/.tmprFYEpR/Y1I6_EF_E__KUE_lz44.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
