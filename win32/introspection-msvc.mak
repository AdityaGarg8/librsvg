# Common NMake Makefile module for checking the build environment is sane
# for building introspection files under MSVC/NMake.
# This can be copied from $(gi_srcroot)\win32 for GNOME items
# that support MSVC builds and introspection under MSVC.

# Can override with env vars as needed
# You will need to have built gobject-introspection for this to work.
# Change or pass in or set the following to suit your environment

!if ![setlocal]		&& \
    ![set PFX_LIB=$(LIBDIR)\lib]	&& \
    ![for %P in (%PFX_LIB%) do @echo PREFIX_LIB_FULL=%~dpnfP > pfx.x]
!endif
!include pfx.x

!if "$(PKG_CONFIG_PATH)" == ""
PKG_CONFIG_PATH=$(PREFIX_LIB_FULL)\pkgconfig
!else
PKG_CONFIG_PATH=$(PREFIX_LIB_FULL)\pkgconfig;$(PKG_CONFIG_PATH)
!endif

!if ![del $(ERRNUL) /q/f pfx.x]
!endif

# Path to the pkg-config tool, if not already in the PATH
!if "$(PKG_CONFIG)" == ""
PKG_CONFIG=pkg-config
!endif

# Don't change anything following this line!

GIR_SUBDIR = share\gir-1.0
GIR_TYPELIBDIR = lib\girepository-1.0

!ifndef G_IR_SCANNER
G_IR_SCANNER = $(BINDIR)\g-ir-scanner
!endif
!ifndef G_IR_COMPILER
G_IR_COMPILER = $(BINDIR)\g-ir-compiler.exe
!endif
!ifndef G_IR_INCLUDEDIR
G_IR_INCLUDEDIR = $(BINDIR)\..\$(GIR_SUBDIR)
!endif
!ifndef G_IR_TYPELIBDIR
G_IR_TYPELIBDIR = $(BINDIR)\..\$(GIR_TYPELIBDIR)
!endif

# Used for generating the documentation using gi-docgen
!ifndef GI_DOCGEN
GI_DOCGEN = gi-docgen
!endif

VALID_PKG_CONFIG_PATH = FALSE

MSG_INVALID_PKGCONFIG = You must set or specifiy a valid PKG_CONFIG_PATH
MSG_INVALID_CFG = You need to specify or set CFG to be release or debug to use this Makefile to build the Introspection Files

ERROR_MSG =

BUILD_INTROSPECTION = TRUE

!if ![set PKG_CONFIG_PATH=$(PKG_CONFIG_PATH)]	\
	&& ![$(PKG_CONFIG) --print-errors --errors-to-stdout $(CHECK_GIR_PACKAGE) > pkgconfig.x]	\
	&& ![setlocal]	\
	&& ![set file="pkgconfig.x"]	\
	&& ![FOR %A IN (%file%) DO @echo PKG_CHECK_SIZE=%~zA > pkgconfig.chksize]	\
	&& ![del $(ERRNUL) /q/f pkgconfig.x]
!endif

!include pkgconfig.chksize
!if "$(PKG_CHECK_SIZE)" == "0"
VALID_PKG_CONFIG_PATH = TRUE
!else
VALID_PKG_CONFIG_PATH = FALSE
!endif

!if ![del $(ERRNUL) /q/f pkgconfig.chksize]
!endif

VALID_CFGSET = FALSE
!if "$(CFG)" == "release" || "$(CFG)" == "debug" || "$(CFG)" == "Release" || "$(CFG)" == "Debug"
VALID_CFGSET = TRUE
!endif

!if "$(VALID_PKG_CONFIG_PATH)" != "TRUE"
BUILD_INTROSPECTION = FALSE
ERROR_MSG = $(MSG_INVALID_PKGCONFIG)
!endif

!if "$(VALID_CFGSET)" != "TRUE"
BUILD_INTROSPECTION = FALSE
ERROR_MSG = $(MSG_INVALID_CFG)
!endif
