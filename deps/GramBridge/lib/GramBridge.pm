package GramBridge;
use Lingua::GA::Gramadoir ();
our %GRAM_CACHED;

sub _get {
    $GRAM_CACHED{default} //= Lingua::GA::Gramadoir->new();
}

sub grammatical_errors {
    my ($class, $text) = @_;
    my $g = _get();
    my $aref = $g->grammatical_errors($text);  # returns arrayref
    return wantarray ? @$aref : $aref;         # list in list ctx, ref in scalar ctx
}

1;
