.DEFAULT: c_exe

%.o:	%.c
	gcc -c $< -o $@

%.o:	%.f90
	gfortran -c $< -o $@

# %.o:	%.rs
# 	rustc --emit obj $< -o $@

# %:	src/%.f90
# 	gfortran $< -o $@

c_exe:	src/main_c.o src/triangle.o
	gfortran $+ -o $@

clean:
	rm -rf src/*.o c_exe
