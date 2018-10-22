<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Enquiry transaction fee/cost from western union IMT GW for current amount,current and country</description>
   <name>FeeInquiry</name>
   <tag></tag>
   <elementGuidId>40d0cc10-9582-4139-821e-0d03a6f9e7f4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;ServiceRequest\&quot;: {\n    \&quot;Header\&quot;: {\n      \&quot;RequestRefID\&quot;: \&quot;qkwestewa\&quot;,\n      \&quot;Timestamp\&quot;: \&quot;201304021534kl5\&quot;,\n      \&quot;OperationName\&quot;: \&quot;IMTFeeInquiry\&quot;,\n      \&quot;OperationVersion\&quot;: \&quot;1\&quot;,\n      \&quot;SourceSystem\&quot;: \&quot;SFCAPP\&quot;,\n      \&quot;Credentials\&quot;: {\n        \&quot;UserName\&quot;: \&quot;OdiDance\&quot;,\n        \&quot;Password\&quot;: \&quot;TDN0bTMxbjIwMTcwOTI0MDM0NTM1ODcz\&quot;\n      }\n    },\n    \&quot;Body\&quot;: {\n      \&quot;Data\&quot;: [\n        {\&quot;Key\&quot;: \&quot;countryiso\&quot;, \&quot;Value\&quot;: \&quot;KE\&quot;},\n        {\&quot;Key\&quot;: \&quot;amount\&quot;, \&quot;Value\&quot;: \&quot;66000\&quot;},\n        {\&quot;Key\&quot;: \&quot;currencyiso\&quot;, \&quot;Value\&quot;: \&quot;KES\&quot;},\n        {\&quot;Key\&quot;: \&quot;msisdn\&quot;, \&quot;Value\&quot;: \&quot;254705912645\&quot;}\n       ]\n    }\n  }\n}\n&quot;,
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
   <restUrl>http://10.184.38.63:15535/feeinquiry</restUrl>
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
