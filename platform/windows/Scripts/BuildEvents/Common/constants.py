LOGGER_DATE_FORMAT = '%m/%d/%Y %I:%M:%S %p'
LOGGER_FORMAT = '%(asctime)s - %(levelname)s : (%(module)s) %(message)s'
ARCH_TO_FOLDER_MAP = {
    'i686-pc-windows-msvc': 'x86',
    'x86_64-pc-windows-msvc': 'x64',
    'aarch64-pc-windows-msvc': 'arm64'
    }
FILES = ['AdGuardFLM.dll', 'AdGuardFLM.pdb']
SRC_DIR_TEMPLATE = "{}/../../target/{}/release"