use ExtUtils::MakeMaker;

WriteMakefile(
    NAME         => 'GramBridge',
    VERSION      => '0.01',
    AUTHOR       => 'caffalaughrey',
    ABSTRACT     => 'Bridge module for Lingua::GA::Gramadoir',
    LICENSE      => 'perl',
    PREREQ_PM    => {
        'Lingua::GA::Gramadoir' => 0,
    },
    PM           => {
        'lib/GramBridge.pm' => '$(INST_LIBDIR)/GramBridge.pm',
    },
);
