import java.nio.CharBuffer;

public final class Taschenrechner {
    private static class SyntaxException extends Exception {
        private String input;
        private int pos;
        private String expected;

        private SyntaxException(CharBuffer input, String expected) {
            super(getShortMessage(input, expected));
            this.pos = input.position();
            input.rewind();
            this.input = input.toString();
            this.expected = expected;
        }

        private static String getShortMessage(CharBuffer input, String expected) {
            int pos = input.position();
            input.rewind();
            String inString = input.toString();
            input.position(pos);

            return getShortMessage(pos, expected, inString);
        }

        private static String getShortMessage(int pos, String expected, String input) {
            StringBuilder sb = new StringBuilder();
            if (pos == input.length()) {
                sb.append("Error: Missing character at end of input, expected: ")
                    .append(expected);
            } else {
                sb.append("Error at column ").append(pos)
                    .append(": Got '").append(input.charAt(pos))
                    .append("' expected: ").append(expected);
            }
            return sb.toString();
        }

        public String getErrorString() {
            StringBuilder sb = new StringBuilder();
            sb.append(input).append('\n');

            for (int i = 0; i < pos; i++) 
                sb.append(' ');
            sb.append("^\n");
        
            sb.append(getShortMessage(pos, expected, input));
            return sb.toString();
        }
    }

    private static double stateS(double accum, CharBuffer input) throws SyntaxException {
        accum = stateE(accum, input);

        if (input.remaining() == 1 && input.get() == '#') {
            return accum;
        } else {
            throw new SyntaxException(input, "<EOF>");
        }
    }
    
    private static double stateE(double accum, CharBuffer input) throws SyntaxException {
        accum = stateP(accum, input);
        accum = stateE2(accum, input);
        return accum;
    }
    
    private static double stateE2(double accum, CharBuffer input) throws SyntaxException {
        switch (input.get()) {
            case '+':
                accum += stateP(accum, input);
                return stateE2(accum, input);
            case '-':
                accum -= stateP(accum, input);
                return stateE2(accum, input);
            default:
                input.position(input.position()-1);
                return accum;
        }
    }
    
    private static double stateP(double accum, CharBuffer input) throws SyntaxException {
        accum = stateT(accum, input);
        accum = stateP2(accum, input);
        return accum;
    }
    
    private static double stateP2(double accum, CharBuffer input) throws SyntaxException {
        switch (input.get()) {
            case '*':
                accum *= stateT(accum, input);
                return stateP2(accum, input);
            case '/':
                accum /= stateT(accum, input);
                return stateP2(accum, input);
            default:
                input.position(input.position()-1);
                return accum;
        }
    }
    
    private static double stateT(double accum, CharBuffer input) throws SyntaxException {
        switch (input.get()) {
            case '-':
                return -stateT(accum, input);
            default:
                input.position(input.position()-1);
                accum = stateF(accum, input);
                return stateT2(accum, input);
        }
    }
    
    private static double stateT2(double accum, CharBuffer input) throws SyntaxException {
        switch (input.get()) {
            case '^':
                return Math.pow(accum, stateT(accum, input));
            default:
                input.position(input.position()-1);
                return accum;
        }
    }
    
    private static double stateF(double accum, CharBuffer input) throws SyntaxException {
        switch (input.get()) {
            case '(':
                accum = stateE(accum, input);
    
                // absolute get, avoid position increase
                if (input.get(input.position()) != ')')
                    throw new SyntaxException(input, ")");
                else
                    input.get(); // Increase position
    
                return accum;
            default:
                input.position(input.position()-1);
                return stateZ(accum, input);
        }
    }
    
    private static double stateZ(double accum, CharBuffer input) throws SyntaxException {
        char current = input.get(input.position());

        if (current != '.' && (current < '1' || current > '9'))
            throw new SyntaxException(input, "<NON-ZERO-DIGIT>");
        
        double number = 0;
        double frac = 0;
    
        while (true) {
            current = input.get();
            if (current < '0' || current > '9')
                break;
            number = number * 10 + (current - '0');
        }
    
        if (current == '.') {
            double div = 10;
            current = input.get(input.position());
    
            if (current < '0' || current > '9')
                throw new SyntaxException(input, "<DIGIT>");
            
            while (true) {
                current = input.get();
                if (current < '0' || current > '9')
                    break;
                frac = frac + (current - '0') / div;
                div *= 10;
            }
        }

        input.position(input.position()-1);
        return number + frac;
    }

    public static double parse(String s) throws SyntaxException {
        CharBuffer buf = CharBuffer.wrap(s + "#");
        return stateS(0, buf);
    }
    
    public static void main(String[] args) {
        try {
            if (args.length == 0) {
                System.err.println("Missing input");
                System.exit(1);
            } else {
                // Geht auch sauberer via Regex
                String input = args[0].replace(" ", "").replace("\t", "").replace("\n", "").replace("\r", "");
                System.out.println(parse(input));
            }
        } catch (SyntaxException ex) {
            ex.printStackTrace();
            System.err.println(ex.getErrorString());
            System.exit(-1);
        }
    }
}
