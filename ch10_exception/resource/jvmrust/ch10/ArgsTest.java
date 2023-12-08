package jvmrust.ch10;

import java.util.Arrays;

public class ArgsTest {

    public static void main(String[] args) {
        foo(args);
    }

    private static void foo(String[] args) {
        try {
            bar(args);
        } catch (NumberFormatException e) {
            System.out.println("NumberFormatException: " + e.getMessage());
        }
    }

    private static void bar(String[] args) {
        if (args.length == 0) {
            throw new IndexOutOfBoundsException("No args!");
        }
        System.out.println(Arrays.toString(args));
        boolean x = isNumeric(args[0]);
        System.out.println(x);
    }

    private static boolean isNumeric(String str) throws NumberFormatException {
        if (str != null && str.matches("[0-9.]+")) {
            return true;
        }
        throw new NumberFormatException("Not number!");
    }

}
