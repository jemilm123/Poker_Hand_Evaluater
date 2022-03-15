pub fn deal(perm:[u32;9])->Vec<String>{
    let mut firstHand=&mut [perm[0], perm[2], perm[4], perm[5], perm[6], perm[7],perm[8]];
    let mut secondHand=&mut [perm[1], perm[3], perm[4], perm[5], perm[6], perm[7],perm[8]];
    let mut winner=winningHand(firstHand, secondHand);
    let winnerRank=handStrength(winner);
    let mut winningNums=extractWinningHand(winner,winnerRank);
    let mut winningNums2=winningNums.to_vec();
    winningNums2.retain(|&i|i  != 0);
    return attachSuits(winner,winningNums2);
}

fn attachSuits(original: &mut [u32;7], winning: Vec<u32>) -> Vec<String>{
    let mut hand: Vec<u32>=original.to_vec();
    let mut returnVec: Vec<String>=Vec::new();
    for x in 0..winning.len(){
        if cardinHand(hand.clone(),winning[x])==true{
            let index = hand.iter().position(|a| *a == winning[x]).unwrap();
            hand.remove(index);
            returnVec.push(format!("{}C",winning[x]));
        }
        else if cardinHand(hand.clone(),winning[x]+13)==true{
            let index = hand.iter().position(|a| *a == winning[x]+13).unwrap();
            hand.remove(index);
            returnVec.push(format!("{}D",winning[x]));
        }
        else if cardinHand(hand.clone(),winning[x]+26)==true{
            let index = hand.iter().position(|a| *a == winning[x]+26).unwrap();
            hand.remove(index);
            returnVec.push(format!("{}H",winning[x]));
        }
        else{
            let index = hand.iter().position(|a| *a == winning[x]+39).unwrap();
            hand.remove(index);
            returnVec.push(format!("{}S",winning[x]));
        }
    }
    return returnVec;
}

fn extractWinningHand(list: &mut [u32;7], strength: u32) -> [u32;5]{
    if strength==10{
        return [10,11,12,13,1];
    }
    else if strength==9{
        return extractSF(list);
    }
    else if strength==8{
        return extract4K(list);
    }
    else if strength==7{
        return extractFH(list);
    }
    else if strength==6{
        return extractFlush(list);
    }
    else if strength==5{
        return extractStraight(list);
    }
    else if strength==4{
        return extract3K(list);
    }
    else if strength==3{
        return extract2P(list);
    }
    else if strength==2{
        return extract1P(list);
    }
    else{
        return extractHigh(list);
    }
}

fn extractSF(list: &mut [u32;7]) -> [u32;5]{
    let mut extraction=[0,0,0,0,0];
    let mut cl: &mut [u32;7]=&mut [0,0,0,0,0,0,0];
    for y in 0..list.len(){
        cl[y]=list[y];
    }
    let mut convertedHand=sortHand(convertHandValues(cl));
    let mut x=&convertedHand[2..7];
    let mut y=&convertedHand[1..6];
    let mut z=&convertedHand[0..5];
    let mut found=false;
    for a in 1..5{
        if x[a]-x[a-1]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        for num in 0..x.len(){
            extraction[num]=x[num];
        }
        return extraction;
    }
    for b in 1..5{
        if y[b]-y[b-1]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        for num in 0..y.len(){
            extraction[num]=y[num];
        }
        return extraction;
    }
    else{
        for num in 0..z.len(){
            extraction[num]=z[num];
        }
        return extraction;
    }
}

fn extract4K(list: &mut [u32;7]) -> [u32;5]{
    let mut repeatingNum=getRepeatCard(list,4);
    return [repeatingNum,repeatingNum,repeatingNum,repeatingNum,0];
}

fn extractFH(list: &mut [u32;7]) -> [u32;5]{
    let mut repeatingT=getRepeatCard(list,3);
    let mut repeatingP=getRepeatCard2(list,2);
    return [repeatingT,repeatingT,repeatingT,repeatingP,repeatingP];
}

fn extractFlush(list: &mut [u32;7]) -> [u32;5]{
    let mut extraction=[0,0,0,0,0];
    let mut suitList: [u32;7]=[0,0,0,0,0,0,0];
    let sortedH=sortHand(list);
    for x in 0..sortedH.len(){
        suitList[x]=getSuit(sortedH[x]);
    }
    let mut repeatedSuit=getRepeatCard(&mut suitList,5);
    for y in (0..7).rev(){
        if getSuit(sortedH[y])==repeatedSuit{
            if extraction[4]==0{
                extraction[4]=sortedH[y];
            }
            else if extraction[3]==0{
                extraction[3]=sortedH[y];
            }
            else if extraction[2]==0{
                extraction[2]=sortedH[y];
            }
            else if extraction[1]==0{
                extraction[1]=sortedH[y];
            }
            else if extraction[0]==0{
                extraction[0]=sortedH[y];
            }
        }
    }
    for z in 0..extraction.len(){
        extraction[z]=getCardValue(extraction[z]);
    }
    return extraction;
}

fn extractStraight(list: &mut [u32;7]) -> [u32;5]{
    return extractSF(list);
}

fn extract3K(list: &mut [u32;7]) -> [u32;5]{
    let mut repeatingNum=getRepeatCard(list,3);
    return [repeatingNum,repeatingNum,repeatingNum,0,0];
}

fn extract2P(list: &mut [u32;7]) -> [u32;5]{
    let mut a=&mut [0,0];
    let mut pairs=get2PCard(list, a);
    let mut high=pairs[1];
    let mut low=pairs[0];
    return [low,low,high,high,0];
}

fn extract1P(list: &mut [u32;7]) -> [u32;5]{
    let mut repeatingNum=(getRepeatCard(list,2));
    return [repeatingNum,repeatingNum,0,0,0];
}

fn extractHigh(list: &mut [u32;7]) -> [u32;5]{
    let mut repeatingNum=getHighCard(list);
    return [repeatingNum,0,0,0,0];
}

fn getCardValue(card: u32) -> u32{
    if card%13==0{
        return 13;
    }
    else{
        return card%13;
    }
}

fn convertHandValues(list: &mut [u32;7]) -> &mut [u32;7]{
    for i in 0..list.len(){
        list[i]=getCardValue(list[i]);
    }
    return list;
}

fn sortHand(list: &mut [u32;7]) -> &mut [u32;7]{
    list.sort();
    return list;
}

fn getSuit(card: u32) -> u32{
    if (card%13)==0{
        return card/13;
    }
    return (card/13)+1;
}

fn SequenceChecker(list: &mut [u32;7]) -> bool{
    let mut converted=sortHand(convertHandValues(list));
    converted=removeDuplicates(converted);
    let mut found=false;
    for a in 0..4{
        if converted[a+1]-converted[a]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        return found;
    }
    for a in 1..5{
        if converted[a+1]-converted[a]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        return found;
    }
    for a in 2..6{
        if converted[a+1]-converted[a]==1{
            found=true;
        }
        else{
            return false;
        }
    }
    if found==true{
        return found;
    }
    return found;
}

fn removeDuplicates(list: &mut [u32;7]) -> &mut [u32;7]{
    for x in 1..list.len(){
        if list[x]==list[x-1]{
            list[x-1]=100;
        }
    }
    return sortHand(list);
}

fn cardCounter(num: u32, list: &mut [u32;7]) -> bool{
    let mut counter=[0,0,0,0,0,0,0];
    let mut converted=convertHandValues(list);
    for a in 0..7{
        for b in 0..7{
            if converted[b]==converted[a]{
                counter[a]+=1;
            }
        }
    }
    
    if num==2{
        let mut two=0;
        for c in 0..counter.len(){
            if counter[c]==2{
                two=two+1;
            }
        }
        if two==4{
            return true;
        }
        else{
            return false;
        }
    }

    if num==1{
        for d in 0..counter.len(){
            if counter[d]==2{
                return true;
            }
        }
        return false;
    }
    for x in 0..counter.len(){
        if counter[x]>=num{
            return true;
        }
    }
    return false;
}

fn StraightSuitCounter(list: &mut [u32;7]) -> bool{
    let mut counter=[0,0,0,0];
    for x in 0..list.len(){
        if getSuit(list[x])==1{
            counter[0]=counter[0]+1;
        }
        else if getSuit(list[x])==2{
            counter[1]=counter[1]+1;
        }
        else if getSuit(list[x])==3{
            counter[2]=counter[2]+1;
        }
        else{
            counter[3]=counter[3]+1;
        }
    }
    for y in 0..counter.len(){
        if counter[y]>=5{
            return true;
        }
    }
    return false;
}

fn checkRF(list: &mut [u32;7]) -> bool{
    if StraightSuitCounter(list)==true{
        let mut x=sortHand(convertHandValues(list));
        if (x[0]==1 && x[3]==10 && x[4]==11 && x[5]==12 && x[6]==13){
            return true;
        }
        else{
            return false;
        }
    }
    else{
        return false; 
    }
}

fn checkSF(list: &mut [u32;7]) -> bool{
    let x=StraightSuitCounter(list);
    if SequenceChecker(list)==true && x==true{
        return true;
    }
    else{
        return false;
    }
}

fn checkFK(list: &mut [u32;7]) -> bool{
    
    if cardCounter(4,list)==true{
        return true;
    }
    else{
        return false;
    }
}

fn checkFH(list: &mut [u32;7]) -> bool{
    if cardCounter(3,list)==true && cardCounter(1,list){
        return true;
    }
    else{
        return false;
    }
}

fn checkFlush(list: &mut [u32;7]) -> bool{
    if StraightSuitCounter(list)==true{
        return true;
    }
    else{
        return false;
    }
}

fn checkStraight(list: &mut [u32;7]) -> bool{
    let x=StraightSuitCounter(list);
    if x==false && SequenceChecker(list)==true{
        return true;
    }
    else{
        return false
    }
}

fn checkTK(list: &mut [u32;7]) -> bool{
    if cardCounter(3,list)==true && cardCounter(2,list)==false && cardCounter(1,list)==false{
        return true;
    }
    else{
        return false;
    }
}

fn check2P(list: &mut [u32;7]) -> bool{
    if cardCounter(2,list)==true{
        return true;
    }
    else{
        return false;
    }
}

fn check1P(list: &mut [u32;7]) -> bool{
    if cardCounter(1,list)==true{
        return true;
    }
    else{
        return false;
    }
}

fn getHighCard(list: &mut [u32;7]) -> u32{
    let mut converted=sortHand(convertHandValues(list));
    return converted[6];
}

fn handStrength(list: &mut [u32;7]) -> u32{
    let mut x: &mut [u32;7]=&mut [0,0,0,0,0,0,0];
    for y in 0..list.len(){
        x[y]=list[y];
    }
    if checkRF(&mut x.clone())==true{
        return 10;
    }
    else if (checkSF(&mut x.clone())==true){
        return 9;
    }
    else if checkFK(&mut x.clone())==true{
        return 8;
    }
    else if checkFH(&mut x.clone())==true{
        return 7;
    }
    else if checkFlush(&mut x.clone())==true{
        return 6;
    }
    else if checkStraight(&mut x.clone())==true{
        return 5;
    }
    else if checkTK(&mut x.clone())==true{
        return 4;
    }
    else if check2P(&mut x.clone())==true{
        return 3;
    }
    else if check1P(&mut x.clone())==true{
        return 2;
    }
    return 1;
}

fn cardRepeat2(list: &mut [u32;7], repeat: u32, card: u32) -> bool{
    let mut counter=0;
    for x in 0..list.len(){
        if getCardValue(list[x])==card{
            counter=counter+1;
        }
    }
    if counter==repeat{
        return true;
    }
    else{
        return false;
    }
}

fn getRepeatCard2(list: &mut [u32;7],repeat: u32) -> u32{
    for x in 0..list.len(){
        let mut convertedCard=getCardValue(list[x]);
        if cardRepeat2(list,repeat,convertedCard)==true{
            return convertedCard;
        }
    }
    return 53;
}

fn cardRepeat(list: &mut [u32;7], repeat: u32, card: u32) -> bool{
    let mut counter=0;
    for x in 0..list.len(){
        if getCardValue(list[x])==card{
            counter=counter+1;
        }
    }
    if counter>=repeat{
        return true;
    }
    else{
        return false;
    }
}

fn getRepeatCard(list: &mut [u32;7],repeat: u32) -> u32{
    for x in 0..list.len(){
        let mut convertedCard=getCardValue(list[x]);
        if cardRepeat(list,repeat,convertedCard)==true{
            return convertedCard;
        }
    }
    return 53;
}

fn get2PCard<'a>(list: &'a mut [u32;7],rList: &'a mut [u32;2]) -> &'a mut [u32;2]{
    let mut cl: &mut [u32;7]=&mut [0,0,0,0,0,0,0];
    for y in 0..list.len(){
        cl[y]=list[y];
    }
    let mut sortedL=sortHand(convertHandValues(cl));
    for x in 1..sortedL.len(){
        if sortedL[x]==sortedL[x-1]{
            if rList[0]==0{
                rList[0]=sortedL[x];
            }
            else{
                rList[1]=sortedL[x];
            }
        }
    }
    return rList;
}

fn getHighinStraight(list: &mut [u32;7]) -> u32{
    let mut sortedL=sortHand(convertHandValues(list));
    let mut x=&sortedL[0..5];
    let mut y=&sortedL[1..6]; 
    let mut z=&sortedL[2..7]; 
    let mut found=false;
    for a in 1..x.len(){
        if x[a]-x[a-1]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        return x[4];
    }
    for b in 1..y.len(){
        if y[b]-y[b-1]==1{
            found=true;
        }
        else{
            found=false;
            break;
        }
    }
    if found==true{
        return y[4];
    }
    else{
        return z[4];
    }
}

fn cardinHand(list: Vec<u32>,card: u32) -> bool{
    for x in 0..list.len(){
        let mut x=list[x];
        if x==card{
            return true;
        }
    }
    return false;
}

fn tieBreakHigh<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    let mut sorted1=sortHand(convertHandValues(list1));
    let mut sorted2=sortHand(convertHandValues(list2));
    for x in (0..7).rev(){
        if sorted1[x]>sorted2[x]{
            return list1;
        }
        else if sorted1[x]<sorted2[x]{
            return list2;
        }
    }
    return list2;

}

fn tieBreak1P<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    let mut first=getRepeatCard(list1,2);
    let mut second=getRepeatCard(list2,2);
    if first==1{
        return list1;
    }
    if second==1{
        return list2;
    }
    if first>second{
        return list1;
    }
    else if first<second{
        return list2;
    }
    return tieBreakHigh(list1,list2);
}

fn tieBreak2P<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    let mut a=&mut [0,0];
    let mut b=&mut [0,0];
    let mut hand1Pairs=get2PCard(list1,a);
    let mut hand2Pairs=get2PCard(list2,b);
    let mut pair1High=hand1Pairs[1];
    let mut pair2High=hand2Pairs[1];
    let mut pair1Low=hand1Pairs[0];
    let mut pair2Low=hand2Pairs[0];
    if pair1High>pair2High{
        return list1;
    }
    else if pair1High<pair2High{
        return list2;
    }
    else if pair1Low>pair2Low{
        return list1;
    }
    else if pair1Low<pair2Low{
        return list2;
    }
    return tieBreakHigh(list1,list2);

}

fn tieBreakTK<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    if getRepeatCard(list1,3) > getRepeatCard(list2,3){
        return list1;
    }
    else{
        return list2;
    } 
}

fn tieBreak4K<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    if getRepeatCard(list1,4)>getRepeatCard(list2,4){
        return list1;
    }
    else{
        return list2;
    }
}

fn tieBreakFH<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    return tieBreakTK(list1,list2);
}

fn tieBreakFlush<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    return tieBreakHigh(list1,list2);
}

fn tieBreakStraight<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    let mut newL1=removeDuplicates(sortHand(convertHandValues(list1)));
    let mut newL2=removeDuplicates(sortHand(convertHandValues(list2)));
    let mut high1=getHighinStraight(newL1);
    let mut high2=getHighinStraight(newL2);
    if high1>high2{
        return list1;
    }
    else{
        return list2;
    }
}

fn tieBreakSF<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    return tieBreakStraight(list1,list2);
}

fn tieBreaker<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7], strength: u32) -> &'a mut [u32;7]{
    if strength==9{
        return tieBreakSF(list1,list2);
    }
    else if strength==8{
        return tieBreak4K(list1,list2);
    }
    else if strength==7{
        return tieBreakFH(list1,list2);
    }
    else if strength==6{
        return tieBreakFlush(list1,list2);
    }
    else if strength==5{
        return tieBreakStraight(list1,list2);
    }
    else if strength==4{
        return tieBreakTK(list1,list2);
    }
    else if strength==3{
        return tieBreak2P(list1,list2);
    }
    else if strength==2{
        return tieBreak1P(list1,list2);
    }
    else{
        return tieBreakHigh(list1,list2);
    }
}

fn winningHand<'a>(list1: &'a mut [u32;7], list2: &'a mut [u32;7]) -> &'a mut [u32;7]{
    let mut hand1=handStrength(list1);
    let mut hand2=handStrength(list2);
    if hand1>hand2{
        return list1;
    }
    else if hand1<hand2{
        return list2;
    }
    else{
        return tieBreaker(list1,list2,hand1);
    }
}
