# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpiPiYZJ/Uz_A079I83LOkrC_tT.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
