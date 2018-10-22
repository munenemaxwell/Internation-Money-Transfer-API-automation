<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>request to initiate an STK Push from IMT tibco via MPESA GW</description>
   <name>PaymentTransaction</name>
   <tag></tag>
   <elementGuidId>39224dfa-b0d0-4aea-8553-120d564dcfd5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ServiceRequest\&quot;: {\n    \&quot;Header\&quot;: {\n      \&quot;RequestRefID\&quot;: \&quot;12122187s2s9121212\&quot;,\n      \&quot;Timestamp\&quot;: \&quot;20130402152345\&quot;,\n      \&quot;OperationName\&quot;: \&quot;IMTPay\&quot;,\n      \&quot;OperationVersion\&quot;: \&quot;1\&quot;,\n      \&quot;SourceSystem\&quot;: \&quot;SFCAPP\&quot;,\n      \&quot;Credentials\&quot;: {\n        \&quot;UserName\&quot;: \&quot;Spiderman\&quot;,\n        \&quot;Password\&quot;: \&quot;a515227d36f6a715a27e\&quot;\n      }\n    },\n    \&quot;Body\&quot;: {\n      \&quot;Data\&quot;: [\n        {\&quot;key\&quot;: \&quot;msisdn\&quot;, \&quot;value\&quot;: \&quot;254714074637\&quot;},\n        {\&quot;key\&quot;: \&quot;amountPayable\&quot;, \&quot;value\&quot;: \&quot;101000\&quot;},\n        {\&quot;key\&quot;: \&quot;trxid\&quot;, \&quot;value\&quot;: \&quot;1001\&quot;},\n        {\&quot;key\&quot;: \&quot;lname\&quot;, \&quot;value\&quot;: \&quot;WAGACIRA\&quot;},\n        {\&quot;key\&quot;: \&quot;countryID\&quot;, \&quot;value\&quot;: \&quot;104\&quot;},\n        {\&quot;key\&quot;: \&quot;stateID\&quot;, \&quot;value\&quot;: \&quot;104\&quot;},\n        {\&quot;key\&quot;: \&quot;currencyID\&quot;, \&quot;value\&quot;: \&quot;20\&quot;}\n      ]\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.184.38.63:15530/getcurrency</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
