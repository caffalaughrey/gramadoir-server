package GramBridge;
use Lingua::GA::Gramadoir ();
use Encode qw(encode decode is_utf8);
our %GRAM_CACHED;

sub _get {
    $GRAM_CACHED{default} //= Lingua::GA::Gramadoir->new();
}

sub grammatical_errors {
    my ($class, $text) = @_;
    my $g = _get();
    my $latin1 = encode('iso-8859-1', is_utf8($text) ? $text : decode('utf-8', $text));
    my $aref = $g->grammatical_errors($latin1);
    my @decoded = map { decode('iso-8859-1', $_) } @$aref;
    return wantarray ? @decoded : \@decoded;
}

1;
