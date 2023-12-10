


// address M
    Or16(a=false,b=AOUT,out[0..14]=addressM);
    And(a=instruction[15], b=instruction[3], out=writeM);

    // The counter

    // control WPC
    And(a=instruction[2], b=outng, out=less);
    And(a=instruction[1], b=outzr, out=eq);
    Not(in=outng, out=notoutng);
    Not(in=outzr, out=notoutzr);
    And(a=notoutng, b=notoutzr, out=gt);
    Or(a=less, b=eq, out=lessOReq);
    Or(a=lessOReq, b=gt, out=condi);
    And(a=condi, b=instruction[15], out=WPC);

    // control PCINC
    Not(in=WPC, out=PCINC);
    
    PC(in=AOUT ,load=WPC ,inc=PCINC ,reset=reset ,out[0..14]=pc);

    