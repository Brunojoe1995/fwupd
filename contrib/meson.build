subdir('firmware_packager')
if get_option('qubes')
  subdir('qubes')
endif

con2 = configuration_data()
con2.set('FWUPD_VERSION', fwupd_version)
con2.set('VERSION', fwupd_version)
con2.set('SOURCE_VERSION', source_version)

configure_file(
  input: 'fwupd.spec.in',
  output: 'fwupd.spec.in',
  configuration: con2,
)

configure_file(
  input: 'bom.json.in',
  output: 'bom.json',
  configuration: con2,
)

if host_machine.system() == 'windows'
  configure_file(
    input: 'fwupd.wxs.in',
    output: 'fwupd.wxs',
    configuration: conf
  )
endif
