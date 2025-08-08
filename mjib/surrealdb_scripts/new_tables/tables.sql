-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1:3306
-- Generation Time: Jun 16, 2025 at 04:25 PM
-- Server version: 8.3.0
-- PHP Version: 8.2.18

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `youngbuilders`
--

-- --------------------------------------------------------

--
-- Table structure for table `academicsession`
--

DROP TABLE IF EXISTS `academicsession`;
CREATE TABLE IF NOT EXISTS `academicsession` (
  `idAcademicYear` int NOT NULL AUTO_INCREMENT,
  `description` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `validFrom` date NOT NULL,
  `validTo` date NOT NULL,
  `dateCreation` date NOT NULL,
  `createdBy` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idAcademicYear`),
  UNIQUE KEY `description` (`description`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `assessment`
--

DROP TABLE IF EXISTS `assessment`;
CREATE TABLE IF NOT EXISTS `assessment` (
  `student` int NOT NULL,
  `course` int NOT NULL,
  `typeAssessment` int NOT NULL,
  `dateAssessment` date NOT NULL,
  `description` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `pointsObtained` float NOT NULL,
  `maxPoints` float NOT NULL,
  `juryDecision` int NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`student`,`course`,`typeAssessment`,`dateAssessment`),
  UNIQUE KEY `description` (`description`),
  KEY `typeAssessment` (`typeAssessment`),
  KEY `juryDecision` (`juryDecision`),
  KEY `course` (`course`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `branch`
--

DROP TABLE IF EXISTS `branch`;
CREATE TABLE IF NOT EXISTS `branch` (
  `idbranch` int NOT NULL AUTO_INCREMENT,
  `description` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `validFrom` date NOT NULL,
  `validTo` date NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idbranch`),
  UNIQUE KEY `description` (`description`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `city`
--

DROP TABLE IF EXISTS `city`;
CREATE TABLE IF NOT EXISTS `city` (
  `idCity` int NOT NULL AUTO_INCREMENT,
  `cityname` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `country` int NOT NULL,
  `flagdeletion` set('No','Yes') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idCity`),
  UNIQUE KEY `designation` (`cityname`),
  KEY `country` (`country`)
) ENGINE=InnoDB AUTO_INCREMENT=60 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `city`
--

INSERT INTO `city` (`idCity`, `cityname`, `comments`, `country`, `flagdeletion`) VALUES
(1, 'Albi', '', 27, 'No'),
(2, 'Amiens', '', 27, 'No'),
(3, 'Béziers', '', 27, 'No'),
(4, 'Bordeaux', '', 27, 'No'),
(5, 'Caen', '', 27, 'No'),
(6, 'Carcassonne', '', 27, 'No'),
(7, 'Chartres', '', 27, 'No'),
(8, 'Clermont-Ferrand', '', 27, 'No'),
(9, 'Colmar', '', 27, 'No'),
(10, 'Cugnaux', '', 27, 'No'),
(11, 'Dijon', '', 27, 'No'),
(12, 'Evreux', '', 27, 'No'),
(13, 'Grenoble', '', 27, 'No'),
(14, 'La Rochelle', '', 27, 'No'),
(15, 'Le Havre', '', 27, 'No'),
(16, 'Le Mans', '', 27, 'No'),
(17, 'Lille', '', 27, 'No'),
(18, 'Limoges', '', 27, 'No'),
(19, 'Lyon', '', 27, 'No'),
(20, 'Marseille', '', 27, 'No'),
(21, 'Metz', '', 27, 'No'),
(22, 'Montauba', '', 27, 'No'),
(23, 'Montpellier', '', 27, 'No'),
(24, 'Mulhouse', '', 27, 'No'),
(25, 'Nancy', '', 27, 'No'),
(26, 'Nantes', '', 27, 'No'),
(27, 'Narbonne', '', 27, 'No'),
(28, 'Nice', '', 27, 'No'),
(29, 'Nîmes', '', 27, 'No'),
(30, 'Niort', '', 27, 'No'),
(31, 'Orléans', '', 27, 'No'),
(32, 'Paris IDF 60', '', 27, 'No'),
(33, 'Paris IDF 75', '', 27, 'No'),
(34, 'Paris IDF 77', '', 27, 'No'),
(35, 'Paris IDF 78', '', 27, 'No'),
(36, 'Paris IDF 91', '', 27, 'No'),
(37, 'Paris IDF 92', '', 27, 'No'),
(38, 'Paris IDF 94', '', 27, 'No'),
(39, 'Paris IDF 95', '', 27, 'No'),
(40, 'Perpignan', '', 27, 'No'),
(41, 'Poitiers', '', 27, 'No'),
(42, 'Reims', '', 27, 'No'),
(43, 'Rennes', '', 27, 'No'),
(44, 'Rouen', '', 27, 'No'),
(45, 'Saint-Etienne', '', 27, 'No'),
(46, 'Strasbourg', '', 27, 'No'),
(47, 'Tours', '', 27, 'No'),
(48, 'Troyes', '', 27, 'No'),
(49, 'Bangui', '', 10, 'No'),
(50, 'Bétou', '', 11, 'No'),
(51, 'Brazzaville', '', 11, 'No'),
(52, 'Dolisie', '', 11, 'No'),
(53, 'Kinkala', '', 11, 'No'),
(54, 'Ngoyo', '', 11, 'No'),
(55, 'Nkouikou', '', 11, 'No'),
(56, 'Ouésso', '', 11, 'No'),
(57, 'Pointe Noire', '', 11, 'No'),
(58, 'Tié-Tié', '', 11, 'No'),
(59, 'Kinshasa', '', 12, 'No');

-- --------------------------------------------------------

--
-- Table structure for table `continent`
--

DROP TABLE IF EXISTS `continent`;
CREATE TABLE IF NOT EXISTS `continent` (
  `idContinent` int NOT NULL AUTO_INCREMENT,
  `continentname` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `flagdeletion` set('No','Yes') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idContinent`),
  UNIQUE KEY `designation` (`continentname`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `continent`
--

INSERT INTO `continent` (`idContinent`, `continentname`, `comments`, `flagdeletion`) VALUES
(1, 'Afrique', '', 'No'),
(2, 'Amérique', '', 'No'),
(3, 'Asie', '', 'No'),
(4, 'Caraïbes', '', 'No'),
(5, 'Europe', '', 'No'),
(6, 'ICC Online', '', 'No');

-- --------------------------------------------------------

--
-- Table structure for table `country`
--

DROP TABLE IF EXISTS `country`;
CREATE TABLE IF NOT EXISTS `country` (
  `idCountry` int NOT NULL AUTO_INCREMENT,
  `countryname` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `continent` int NOT NULL,
  `flagDeletion` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idCountry`),
  UNIQUE KEY `designation` (`countryname`),
  KEY `continent` (`continent`)
) ENGINE=InnoDB AUTO_INCREMENT=33 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `country`
--

INSERT INTO `country` (`idCountry`, `countryname`, `comments`, `continent`, `flagDeletion`) VALUES
(1, 'Benin', '', 1, 'No'),
(2, 'Burundi', '', 1, 'No'),
(3, 'Cameroun', '', 1, 'No'),
(4, 'Côte d\'Ivoire', '', 1, 'No'),
(5, 'Gabon', '', 1, 'No'),
(6, 'Guinée', '', 1, 'No'),
(7, 'Madagascar', '', 1, 'No'),
(8, 'Mali', '', 1, 'No'),
(9, 'Maroc', '', 1, 'No'),
(10, 'République Centre Africaine', '', 1, 'No'),
(11, 'République Démocratique du Congo', '', 1, 'No'),
(12, 'République Populaire du Congo', '', 1, 'No'),
(13, 'Rwanda', '', 1, 'No'),
(14, 'Sénégal', '', 1, 'No'),
(15, 'Togo', '', 1, 'No'),
(16, 'Tunise', '', 1, 'No'),
(17, 'Canada', '', 2, 'No'),
(18, 'Etats-Unis', '', 2, 'No'),
(19, 'Emirats Arabes Unis', '', 3, ''),
(20, 'Cuba', '', 4, 'No'),
(21, 'Guadeloupe', '', 4, 'No'),
(22, 'Guyane', '', 4, 'No'),
(23, 'Haïti', '', 4, 'No'),
(24, 'Martinique', '', 4, 'No'),
(25, 'Allemagne', '', 5, 'No'),
(26, 'Belgique', '', 5, 'No'),
(27, 'France', '', 5, 'No'),
(28, 'Luxembourg', '', 5, 'No'),
(29, 'Pays-Bas', '', 5, 'No'),
(30, 'Royaume Uni', '', 5, 'No'),
(31, 'Suisse', '', 5, 'No'),
(32, 'ICC Online', '', 6, 'No');

-- --------------------------------------------------------

--
-- Table structure for table `courses`
--

DROP TABLE IF EXISTS `courses`;
CREATE TABLE IF NOT EXISTS `courses` (
  `idCourse` int NOT NULL AUTO_INCREMENT,
  `designation` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `coursePath` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `validFrom` date NOT NULL,
  `deletionFlag` set('No','Yes') NOT NULL,
  PRIMARY KEY (`idCourse`),
  UNIQUE KEY `designation` (`designation`),
  UNIQUE KEY `coursePath` (`coursePath`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- Table structure for table `coursesdiscipline`
--

DROP TABLE IF EXISTS `coursesdiscipline`;
CREATE TABLE IF NOT EXISTS `coursesdiscipline` (
  `discipline` int NOT NULL,
  `course` int NOT NULL,
  `designation` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`discipline`,`course`),
  UNIQUE KEY `designation` (`designation`),
  KEY `course` (`course`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `discipline`
--

DROP TABLE IF EXISTS `discipline`;
CREATE TABLE IF NOT EXISTS `discipline` (
  `idDiscipline` int NOT NULL AUTO_INCREMENT,
  `designation` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `branch` int NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idDiscipline`),
  UNIQUE KEY `designation` (`designation`),
  KEY `branch` (`branch`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `employmentstatus`
--

DROP TABLE IF EXISTS `employmentstatus`;
CREATE TABLE IF NOT EXISTS `employmentstatus` (
  `idEmploymentStatus` int NOT NULL AUTO_INCREMENT,
  `designation` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idEmploymentStatus`),
  UNIQUE KEY `designation` (`designation`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `jurydecision`
--

DROP TABLE IF EXISTS `jurydecision`;
CREATE TABLE IF NOT EXISTS `jurydecision` (
  `idjuryDecision` int NOT NULL AUTO_INCREMENT,
  `description` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idjuryDecision`),
  UNIQUE KEY `description` (`description`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `potentialsstudents`
--

DROP TABLE IF EXISTS `potentialsstudents`;
CREATE TABLE IF NOT EXISTS `potentialsstudents` (
  `emailAddressMailing` int NOT NULL,
  `emailAddressUsers` int NOT NULL,
  `refused` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  `refusedValidFrom` date NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`emailAddressMailing`,`emailAddressUsers`),
  KEY `emailAddressUsers` (`emailAddressUsers`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `prayerassembly`
--

DROP TABLE IF EXISTS `prayerassembly`;
CREATE TABLE IF NOT EXISTS `prayerassembly` (
  `idPrayerAssembly` int NOT NULL AUTO_INCREMENT,
  `designation` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `flagDeletion` set('No','Yes') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idPrayerAssembly`),
  UNIQUE KEY `designation` (`designation`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `prayerassemblycity`
--

DROP TABLE IF EXISTS `prayerassemblycity`;
CREATE TABLE IF NOT EXISTS `prayerassemblycity` (
  `idPrayerAssembly` int NOT NULL,
  `idCity` int NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `flagdeletion` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idPrayerAssembly`,`idCity`),
  KEY `idCity` (`idCity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `profile`
--

DROP TABLE IF EXISTS `profile`;
CREATE TABLE IF NOT EXISTS `profile` (
  `idprofile` int NOT NULL AUTO_INCREMENT,
  `profilename` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) COLLATE utf8mb4_general_ci DEFAULT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idprofile`),
  UNIQUE KEY `designation` (`profilename`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `profile`
--

INSERT INTO `profile` (`idprofile`, `profilename`, `comments`, `deletionFlag`) VALUES
(1, 'student', NULL, 'No'),
(3, 'Professor', 'Peut solliciter une inscription', '');

-- --------------------------------------------------------

--
-- Table structure for table `registration`
--

DROP TABLE IF EXISTS `registration`;
CREATE TABLE IF NOT EXISTS `registration` (
  `idRegistration` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `student` int NOT NULL,
  `branch` int NOT NULL,
  `academicsession` int NOT NULL,
  `dateCreation` date NOT NULL,
  `createdBy` varchar(45) COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idRegistration`,`student`,`branch`,`academicsession`),
  KEY `academicsession` (`academicsession`),
  KEY `student` (`student`),
  KEY `branch` (`branch`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `typeassessment`
--

DROP TABLE IF EXISTS `typeassessment`;
CREATE TABLE IF NOT EXISTS `typeassessment` (
  `idtypeAssessment` int NOT NULL AUTO_INCREMENT,
  `description` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idtypeAssessment`),
  UNIQUE KEY `description` (`description`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
CREATE TABLE IF NOT EXISTS `users` (
  `idusers` int NOT NULL AUTO_INCREMENT,
  `emailAddress` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `passWord` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `createdFrom` date NOT NULL,
  `dateUpdatedPassword` date DEFAULT NULL,
  `firstName` varchar(45) NOT NULL,
  `name` varchar(45) NOT NULL,
  `birthDay` date NOT NULL,
  `phoneNumber` varchar(45) DEFAULT NULL,
  `residenceCity` varchar(45) NOT NULL,
  `residenceCountry` varchar(45) NOT NULL,
  `prayerAssembly` int NOT NULL,
  `profile` int NOT NULL,
  `professionalStatus` int NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') NOT NULL,
  PRIMARY KEY (`idusers`),
  UNIQUE KEY `emailAddress` (`emailAddress`),
  UNIQUE KEY `passWord` (`passWord`),
  KEY `prayerAssembly` (`prayerAssembly`),
  KEY `profile` (`profile`),
  KEY `professionalStatus` (`professionalStatus`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- Table structure for table `usersmailing`
--

DROP TABLE IF EXISTS `usersmailing`;
CREATE TABLE IF NOT EXISTS `usersmailing` (
  `idUsersMailing` int NOT NULL AUTO_INCREMENT,
  `emailAddress` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `recognisedAsValid` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  `recognisedValidFrom` date NOT NULL,
  `processed` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  `processedValidFrom` date NOT NULL,
  `comments` varchar(45) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `deletionFlag` set('No','Yes') COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`idUsersMailing`),
  UNIQUE KEY `emailAddress` (`emailAddress`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `assessment`
--
ALTER TABLE `assessment`
  ADD CONSTRAINT `assessment_ibfk_1` FOREIGN KEY (`student`) REFERENCES `users` (`idusers`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `assessment_ibfk_2` FOREIGN KEY (`typeAssessment`) REFERENCES `typeassessment` (`idtypeAssessment`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `assessment_ibfk_3` FOREIGN KEY (`juryDecision`) REFERENCES `jurydecision` (`idjuryDecision`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `assessment_ibfk_4` FOREIGN KEY (`course`) REFERENCES `courses` (`idCourse`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `city`
--
ALTER TABLE `city`
  ADD CONSTRAINT `city_ibfk_1` FOREIGN KEY (`country`) REFERENCES `country` (`idCountry`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `country`
--
ALTER TABLE `country`
  ADD CONSTRAINT `country_ibfk_1` FOREIGN KEY (`continent`) REFERENCES `continent` (`idContinent`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `coursesdiscipline`
--
ALTER TABLE `coursesdiscipline`
  ADD CONSTRAINT `coursesdiscipline_ibfk_1` FOREIGN KEY (`discipline`) REFERENCES `discipline` (`idDiscipline`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `coursesdiscipline_ibfk_2` FOREIGN KEY (`course`) REFERENCES `courses` (`idCourse`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `discipline`
--
ALTER TABLE `discipline`
  ADD CONSTRAINT `discipline_ibfk_1` FOREIGN KEY (`branch`) REFERENCES `branch` (`idbranch`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `potentialsstudents`
--
ALTER TABLE `potentialsstudents`
  ADD CONSTRAINT `potentialsstudents_ibfk_1` FOREIGN KEY (`emailAddressUsers`) REFERENCES `users` (`idusers`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `potentialsstudents_ibfk_2` FOREIGN KEY (`emailAddressMailing`) REFERENCES `usersmailing` (`idUsersMailing`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `prayerassemblycity`
--
ALTER TABLE `prayerassemblycity`
  ADD CONSTRAINT `prayerassemblycity_ibfk_1` FOREIGN KEY (`idCity`) REFERENCES `city` (`idCity`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `prayerassemblycity_ibfk_2` FOREIGN KEY (`idPrayerAssembly`) REFERENCES `prayerassembly` (`idPrayerAssembly`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `registration`
--
ALTER TABLE `registration`
  ADD CONSTRAINT `registration_ibfk_1` FOREIGN KEY (`academicsession`) REFERENCES `academicsession` (`idAcademicYear`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `registration_ibfk_2` FOREIGN KEY (`student`) REFERENCES `users` (`idusers`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `registration_ibfk_3` FOREIGN KEY (`branch`) REFERENCES `branch` (`idbranch`) ON DELETE RESTRICT ON UPDATE RESTRICT;

--
-- Constraints for table `users`
--
ALTER TABLE `users`
  ADD CONSTRAINT `users_ibfk_1` FOREIGN KEY (`prayerAssembly`) REFERENCES `prayerassembly` (`idPrayerAssembly`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `users_ibfk_2` FOREIGN KEY (`profile`) REFERENCES `profile` (`idprofile`) ON DELETE RESTRICT ON UPDATE RESTRICT,
  ADD CONSTRAINT `users_ibfk_3` FOREIGN KEY (`professionalStatus`) REFERENCES `employmentstatus` (`idEmploymentStatus`) ON DELETE RESTRICT ON UPDATE RESTRICT;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;