TARGET=spartan
BUILDDIR=build
COBOLDIR=cobol
COBENTRY=entry
LIBCOBOL=${BUILDDIR}/libcobol.so
ENTRY=main.rs
RUSTFILES=$(shell find *.rs -type f)

# Compilers
RC=rustc
COBC=cobc

all: $(BUILDDIR)/$(TARGET)

run: all
	@echo "--------------------------------------------------------------------------------------"
	@echo "RUNNING MAIN TARGET..."
	@echo
	@LD_LIBRARY_PATH=$(BUILDDIR) ./$(BUILDDIR)/$(TARGET)

release: all
	@echo "--------------------------------------------------------------------------------------"
	@echo "BUILDING RELEASE BUILD..."
	@echo
	$(RC) -A dead_code -O -C opt-level=3 -C lto -C link-arg=-s -C link-arg=-L$(BUILDDIR) -C link-arg=-lcobol -C link-arg=-lm -C link-arg=-lpthread -C link-arg=-lstdc++ ${ENTRY} -o $@ -L$(CURDIR)/$(BUILDDIR) -lcobol --test

test: $(BUILDDIR)/test
	@echo "--------------------------------------------------------------------------------------"
	@echo "RUNNING TESTS TARGET..."
	@echo
	@LD_LIBRARY_PATH=$(BUILDDIR) ./$(BUILDDIR)/test --show-output

$(BUILDDIR)/test: ${LIBCOBOL} ${RUSTFILES}
	@echo "--------------------------------------------------------------------------------------"
	@echo "BUILDING TESTS TARGET..."
	@echo
	$(RC) -A dead_code ${ENTRY} -o $@ -L$(CURDIR)/$(BUILDDIR) -lcobol --test

$(BUILDDIR)/$(TARGET): ${LIBCOBOL} ${RUSTFILES}
	@echo "--------------------------------------------------------------------------------------"
	@echo "BUILDING ${ENTRY}..."
	@echo
	mkdir -p $(BUILDDIR)
	$(RC) -A dead_code ${ENTRY} -o $@ -L$(CURDIR)/$(BUILDDIR) -lcobol

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

${LIBCOBOL}: $(shell find ${COBOLDIR}/* -type f)
	@echo "--------------------------------------------------------------------------------------"
	@echo "COMPILING COBOL FILES..."
	@echo
	$(COBC) -fimplicit-init -O -b ${COBOLDIR}/*.cob -o ${LIBCOBOL}

clean:
	rm -rf $(BUILDDIR)/*
	@echo "cleaned!"

