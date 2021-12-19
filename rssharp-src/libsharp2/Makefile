all: lib/libsharp.a
py: pysharp.so


%.o: src/%.c
	$(CC) $< -o $@ -c -I . -fPIC -g -O2

%.o: src/%.cxx
	$(CXX) $< -o $@ -c -I . -fPIC -g -O2


lib/libsharp.a: pocketfft.o sharp_almhelpers.o sharp.o sharp_core.o sharp_core_inc.o sharp_geomhelpers.o sharp_legendre_roots.o sharp_utils.o sharp_ylmgen_c.o
	mkdir -p lib
	$(AR) cr $@ $^ 
	


pysharp.so: py/pysharp.cc lib/libsharp.a
	$(CXX) -o $@ $< -I . `pkg-config --cflags python3` -L ./lib -lsharp  --shared -fPIC -O2

test: map2alm2map_test.cpp lib/libsharp.a
	g++ -o $@ $< -I include -L ./lib -lsharp -lm

clean: 
	$(RM) *.o
