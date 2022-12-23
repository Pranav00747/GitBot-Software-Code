func somerhing_funny_(_ques string, _ans string)
{
  var mg := mongo.Connect(Url("url/ip:27017"))
  var cl := mg.Database("life").Collection("me")
  cl.insertOne(bson.D{{"Question":_ques, "Answere":_ans}})
}

something_funny_("Dad What If I have scored between 90-100 then, What Next ?", "Watch Prisonor of Azkaban Movie Again")
   
  
